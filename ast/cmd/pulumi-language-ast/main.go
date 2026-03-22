package main

import (
	"context"
	"encoding/json"
	"errors"
	"flag"
	"fmt"
	"os"
	"os/signal"
	"path/filepath"
	"strings"
	"time"

	"github.com/andrzejressel/pulumi-ast/codegen/ast"
	"github.com/hashicorp/hcl/v2"
	hclsyntax "github.com/pulumi/pulumi/pkg/v3/codegen/hcl2/syntax"
	"github.com/pulumi/pulumi/pkg/v3/codegen/pcl"
	"github.com/pulumi/pulumi/pkg/v3/codegen/schema"
	"github.com/pulumi/pulumi/sdk/v3/go/common/resource/plugin"
	"github.com/pulumi/pulumi/sdk/v3/go/common/util/cmdutil"
	"github.com/pulumi/pulumi/sdk/v3/go/common/util/logging"
	"github.com/pulumi/pulumi/sdk/v3/go/common/util/rpcutil"
	"github.com/pulumi/pulumi/sdk/v3/go/common/workspace"
	pulumirpc "github.com/pulumi/pulumi/sdk/v3/proto/go"
	"google.golang.org/grpc"
	"google.golang.org/protobuf/types/known/emptypb"
)

const pluginVersion = "dev"

func main() {
	var tracing string
	flag.StringVar(&tracing, "tracing", "", "Emit tracing to a Zipkin-compatible tracing endpoint")
	flag.Parse()

	args := flag.Args()
	logging.InitLogging(false, 0, false)
	cmdutil.InitTracing("pulumi-language-ast", "pulumi-language-ast", tracing)

	var cancelChannel chan bool
	if len(args) > 0 {
		var err error
		cancelChannel, err = setupHealthChecks(args[0])
		if err != nil {
			cmdutil.Exit(fmt.Errorf("could not start health check host RPC server: %w", err))
		}
	}

	handle, err := rpcutil.ServeWithOptions(rpcutil.ServeOptions{
		Cancel: cancelChannel,
		Init: func(srv *grpc.Server) error {
			pulumirpc.RegisterLanguageRuntimeServer(srv, &astLanguageHost{})
			return nil
		},
		Options: rpcutil.OpenTracingServerInterceptorOptions(nil),
	})
	if err != nil {
		cmdutil.Exit(fmt.Errorf("could not start language host RPC server: %w", err))
	}

	fmt.Printf("%d\n", handle.Port)

	if err := <-handle.Done; err != nil {
		cmdutil.Exit(fmt.Errorf("language host RPC stopped serving: %w", err))
	}
}

func setupHealthChecks(engineAddress string) (chan bool, error) {
	ctx, cancel := signal.NotifyContext(context.Background(), os.Interrupt)

	cancelChannel := make(chan bool)
	go func() {
		<-ctx.Done()
		cancel()
		close(cancelChannel)
	}()

	err := rpcutil.Healthcheck(ctx, engineAddress, 5*time.Minute, cancel)
	if err != nil {
		return nil, err
	}

	return cancelChannel, nil
}

type astLanguageHost struct {
	pulumirpc.UnimplementedLanguageRuntimeServer
}

func (host *astLanguageHost) GetPluginInfo(context.Context, *emptypb.Empty) (*pulumirpc.PluginInfo, error) {
	return &pulumirpc.PluginInfo{Version: pluginVersion}, nil
}

func (host *astLanguageHost) GetRequiredPackages(
	context.Context,
	*pulumirpc.GetRequiredPackagesRequest,
) (*pulumirpc.GetRequiredPackagesResponse, error) {
	return &pulumirpc.GetRequiredPackagesResponse{Packages: []*pulumirpc.PackageDependency{}}, nil
}

func (host *astLanguageHost) GenerateProgram(
	_ context.Context,
	req *pulumirpc.GenerateProgramRequest,
) (*pulumirpc.GenerateProgramResponse, error) {
	loader, err := schema.NewLoaderClient(req.LoaderTarget)
	if err != nil {
		return nil, err
	}
	defer loader.Close()
	files, diags, err := generateProgramFromSource(req.Source, schema.NewCachedLoader(loader), req.Strict)
	if err != nil {
		return nil, err
	}

	if true {
		return nil, fmt.Errorf("FILES: %s", files)
	}

	return &pulumirpc.GenerateProgramResponse{
		Source:      files,
		Diagnostics: plugin.HclDiagnosticsToRPCDiagnostics(diags),
	}, nil
}

func (host *astLanguageHost) GenerateProject(_ context.Context, req *pulumirpc.GenerateProjectRequest) (*pulumirpc.GenerateProjectResponse, error) {
	loader, err := schema.NewLoaderClient(req.LoaderTarget)
	if err != nil {
		return nil, err
	}

	var extraOptions []pcl.BindOption
	if !req.Strict {
		extraOptions = append(extraOptions, pcl.NonStrictBindOptions()...)
	}

	extraOptions = append(extraOptions, pcl.PreferOutputVersionedInvokes)

	program, diags, err := pcl.BindDirectory(req.SourceDirectory, loader, extraOptions...)
	if err != nil {
		return nil, err
	}

	if diags.HasErrors() {
		rpcDiagnostics := plugin.HclDiagnosticsToRPCDiagnostics(diags)
		return &pulumirpc.GenerateProjectResponse{
			Diagnostics: rpcDiagnostics,
		}, nil
	}

	if program == nil {
		return nil, errors.New("internal error: program was nil")
	}

	var project workspace.Project
	if err := json.Unmarshal([]byte(req.Project), &project); err != nil {
		return nil, err
	}

	err = generateProject(req.SourceDirectory, project, program)
	if err != nil {
		return nil, fmt.Errorf("could not generate project: %w", err)
	}

	return &pulumirpc.GenerateProjectResponse{}, nil

}
func (host *astLanguageHost) GeneratePackage(_ context.Context, req *pulumirpc.GeneratePackageRequest) (*pulumirpc.GeneratePackageResponse, error) {

	loader, err := schema.NewLoaderClient(req.LoaderTarget)
	if err != nil {
		return nil, err
	}

	var spec schema.PackageSpec
	err = json.Unmarshal([]byte(req.Schema), &spec)
	if err != nil {
		return nil, err
	}

	diags := hcl.Diagnostics{}
	pkg, bindDiags, err := schema.BindSpec(spec, loader, schema.ValidationOptions{
		AllowDanglingReferences: true,
	})
	if err != nil {
		return nil, err
	}
	diags = diags.Extend(bindDiags)
	if bindDiags.HasErrors() {
		return &pulumirpc.GeneratePackageResponse{
			Diagnostics: plugin.HclDiagnosticsToRPCDiagnostics(diags),
		}, nil
	}

	files, generationDiags, err := ast.GenerateJSONPackage(pkg)
	if err != nil {
		return nil, err
	}
	diags = diags.Extend(generationDiags)

	if err = os.MkdirAll(req.Directory, 0o755); err != nil {
		return nil, fmt.Errorf("could not create package output directory %q: %w", req.Directory, err)
	}

	for fileName, contents := range files {
		target := filepath.Join(req.Directory, fileName)
		if err = os.WriteFile(target, contents, 0o644); err != nil {
			return nil, fmt.Errorf("could not write package file %q: %w", target, err)
		}
	}

	return &pulumirpc.GeneratePackageResponse{
		Diagnostics: plugin.HclDiagnosticsToRPCDiagnostics(diags),
	}, nil
}

func (host *astLanguageHost) Pack(ctx context.Context, request *pulumirpc.PackRequest) (*pulumirpc.PackResponse, error) {
	return &pulumirpc.PackResponse{}, nil
	//return nil, status.Errorf(codes.Unimplemented, "method Pack not implemented")
}

func generateProgramFromSource(
	source map[string]string,
	loader schema.ReferenceLoader,
	strict bool,
) (map[string][]byte, hcl.Diagnostics, error) {
	parser := hclsyntax.NewParser()
	for path, contents := range source {
		err := parser.ParseFile(strings.NewReader(contents), path)
		if err != nil {
			return nil, nil, err
		}
	}
	if parser.Diagnostics.HasErrors() {
		return nil, parser.Diagnostics, nil
	}

	bindOptions := []pcl.BindOption{
		pcl.Loader(loader),
		pcl.PreferOutputVersionedInvokes,
	}
	if !strict {
		bindOptions = append(bindOptions, pcl.NonStrictBindOptions()...)
	}

	program, diags, err := pcl.BindProgram(parser.Files, bindOptions...)
	if err != nil {
		return nil, nil, err
	}
	if diags.HasErrors() {
		return nil, diags, nil
	}
	if program == nil {
		return nil, nil, fmt.Errorf("internal error: program was nil")
	}

	files, generationDiags, err := ast.GenerateJSONProgram(program)
	if err != nil {
		return nil, nil, err
	}

	return files, diags.Extend(generationDiags), nil
}

func generateProject(
	directory string,
	project workspace.Project,
	program *pcl.Program,
) error {

	files, generationDiags, err := ast.GenerateJSONProgram(program)
	if err != nil {
		return err
	}

	if generationDiags.HasErrors() {
		return fmt.Errorf("generation diagnostics: %v", generationDiags)
	}

	file := files["main.pcl.json"]

	err = os.WriteFile(fmt.Sprintf("%s/main.pcl.json", directory), file, 0644)
	if err != nil {
		return fmt.Errorf("could not write main.pcl.json: %w", err)
	}

	return nil

}
