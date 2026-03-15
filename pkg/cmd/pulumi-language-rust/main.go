package main

import (
	"context"
	"encoding/json"
	"errors"
	"flag"
	"fmt"
	"os"
	"os/exec"
	"os/signal"
	"path"
	"path/filepath"
	"strings"
	"time"

	"github.com/andrzejressel/pulumi-rust/codegen/rust"
	"github.com/hashicorp/hcl/v2"
	hclsyntax "github.com/pulumi/pulumi/pkg/v3/codegen/hcl2/syntax"
	"github.com/pulumi/pulumi/pkg/v3/codegen/pcl"
	"github.com/pulumi/pulumi/pkg/v3/codegen/schema"
	"github.com/pulumi/pulumi/sdk/v3/go/common/encoding"
	"github.com/pulumi/pulumi/sdk/v3/go/common/resource/plugin"
	"github.com/pulumi/pulumi/sdk/v3/go/common/util/cmdutil"
	"github.com/pulumi/pulumi/sdk/v3/go/common/util/logging"
	"github.com/pulumi/pulumi/sdk/v3/go/common/util/rpcutil"
	"github.com/pulumi/pulumi/sdk/v3/go/common/workspace"
	pulumirpc "github.com/pulumi/pulumi/sdk/v3/proto/go"
	"go.opentelemetry.io/otel"
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
	cmdutil.InitTracing("pulumi-language-rust", "pulumi-language-rust", tracing)

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
			pulumirpc.RegisterLanguageRuntimeServer(srv, &rustLanguageHost{})
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

type rustLanguageHost struct {
	pulumirpc.UnimplementedLanguageRuntimeServer
}

func (host *rustLanguageHost) GetPluginInfo(context.Context, *emptypb.Empty) (*pulumirpc.PluginInfo, error) {
	return &pulumirpc.PluginInfo{Version: pluginVersion}, nil
}

func (host *rustLanguageHost) GetProgramDependencies(context.Context, *pulumirpc.GetProgramDependenciesRequest) (*pulumirpc.GetProgramDependenciesResponse, error) {
	return &pulumirpc.GetProgramDependenciesResponse{}, nil
}

func (host *rustLanguageHost) InstallDependencies(req *pulumirpc.InstallDependenciesRequest, server grpc.ServerStreamingServer[pulumirpc.InstallDependenciesResponse]) error {

	tracer := otel.Tracer("pulumi-language-rust")
	_, otelSpan := cmdutil.StartSpan(server.Context(), tracer, "rust-install-deps")
	defer otelSpan.End()

	closer, stdout, stderr, err := rpcutil.MakeInstallDependenciesStreams(server, req.IsTerminal)
	if err != nil {
		return err
	}
	defer closer.Close()

	directoryName := path.Base(req.Info.ProgramDirectory)

	// intentionally running dynamic program name.
	cmd := exec.Command("cargo", "build") // nolint: gosec
	cmd.Dir = req.Info.ProgramDirectory
	cmd.Stdout = stdout
	cmd.Stderr = stderr
	cmd.Env = append(os.Environ(), fmt.Sprintf("CARGO_TARGET_DIR=/home/andrzej/test_target/%s", directoryName))

	if err := runCommand(cmd); err != nil {
		logging.V(5).Infof("InstallDependencies(Directory=%s): failed", req.Info.ProgramDirectory) //nolint:staticcheck
		return err
	}

	defer closer.Close()

	return nil
	//return status.Errorf(codes.Unimplemented, "method InstallDependencies not implemented")
}

func (host *rustLanguageHost) GetRequiredPackages(
	context.Context,
	*pulumirpc.GetRequiredPackagesRequest,
) (*pulumirpc.GetRequiredPackagesResponse, error) {
	return &pulumirpc.GetRequiredPackagesResponse{Packages: []*pulumirpc.PackageDependency{}}, nil
}

func (host *rustLanguageHost) GenerateProgram(
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

	return &pulumirpc.GenerateProgramResponse{
		Source:      files,
		Diagnostics: plugin.HclDiagnosticsToRPCDiagnostics(diags),
	}, nil
}

func (host *rustLanguageHost) GenerateProject(_ context.Context, req *pulumirpc.GenerateProjectRequest) (*pulumirpc.GenerateProjectResponse, error) {
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

	err = generateProject(req.TargetDirectory, project, program)
	if err != nil {
		return nil, fmt.Errorf("could not generate project: %w", err)
	}

	return &pulumirpc.GenerateProjectResponse{}, nil

}
func (host *rustLanguageHost) GeneratePackage(_ context.Context, req *pulumirpc.GeneratePackageRequest) (*pulumirpc.GeneratePackageResponse, error) {

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

	err = rust.GeneratePackage(pkg, req.Directory)
	if err != nil {
		return nil, err
	}

	return &pulumirpc.GeneratePackageResponse{
		Diagnostics: plugin.HclDiagnosticsToRPCDiagnostics(diags),
	}, nil
}

// Pack Cargo does not have binary deployable packages
func (host *rustLanguageHost) Pack(ctx context.Context, request *pulumirpc.PackRequest) (*pulumirpc.PackResponse, error) {
	return &pulumirpc.PackResponse{}, nil
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

	files, generationDiags, err := rust.GenerateProgram(program)
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
	rootDirectory := directory

	err := rust.GenerateProject(program, rootDirectory)
	if err != nil {
		return err
	}

	// Set the runtime to "java" then marshal to Pulumi.yaml
	project.Runtime = workspace.NewProjectRuntimeInfo("rust", nil)
	projectBytes, err := encoding.YAML.Marshal(project)
	if err != nil {
		return err
	}

	filesWithPackages := make(map[string][]byte)

	filesWithPackages[filepath.Join(rootDirectory, "Pulumi.yaml")] = projectBytes
	//filesWithPackages[filepath.Join(rootDirectory, "Cargo.toml")] = []byte("[package]\nname=\"TEST\"")

	for filePath, data := range filesWithPackages {
		dir := filepath.Dir(filePath)
		err := os.MkdirAll(dir, os.ModePerm)
		if err != nil {
			return fmt.Errorf("could not create output directory %s: %w", dir, err)
		}
		err = os.WriteFile(filePath, data, 0o600)
		if err != nil {
			return fmt.Errorf("could not write output program: %w", err)
		}
	}

	return nil

}
