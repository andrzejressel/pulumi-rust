package main

import (
	"bufio"
	"context"
	"fmt"
	"io"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"sync"
	"testing"

	"github.com/pulumi/pulumi/sdk/v3"
	"github.com/pulumi/pulumi/sdk/v3/go/common/util/contract"
	"github.com/pulumi/pulumi/sdk/v3/go/common/util/rpcutil"
	pulumirpc "github.com/pulumi/pulumi/sdk/v3/proto/go"
	testingrpc "github.com/pulumi/pulumi/sdk/v3/proto/go/testing"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

// TestLanguage runs the language conformance test suite against the PCL AST language host.
func TestLanguage(t *testing.T) {

	if runtime.GOOS == "windows" {
		panic("Windows not yet supported")
	}

	t.Parallel()

	_, engine := runTestingHost(t)

	tests, err := engine.GetLanguageTests(context.Background(), &testingrpc.GetLanguageTestsRequest{})
	require.NoError(t, err)

	cancel := make(chan bool)
	//rootDir := t.TempDir()
	rootDir := "/home/andrzej/MojeProgramy/pulumi-rust/pkg/target"

	handle, err := rpcutil.ServeWithOptions(rpcutil.ServeOptions{
		Init: func(srv *grpc.Server) error {
			pulumirpc.RegisterLanguageRuntimeServer(srv, &rustLanguageHost{})
			return nil
		},
		Cancel: cancel,
	})
	require.NoError(t, err)

	snapshotDir := "./testdata/"

	prepare, err := engine.PrepareLanguageTests(context.Background(), &testingrpc.PrepareLanguageTestsRequest{
		LanguagePluginName:   "rust",
		LanguagePluginTarget: fmt.Sprintf("127.0.0.1:%d", handle.Port),
		TemporaryDirectory:   rootDir,
		SnapshotDirectory:    snapshotDir,
		CoreSdkVersion:       sdk.Version.String(),
		LanguageInfo:         "{}",
	})
	require.NoError(t, err)

	for _, tt := range tests.Tests {
		tt := tt
		t.Run(tt, func(t *testing.T) {
			t.Parallel()

			if expected, ok := expectedFailures[tt]; ok {
				t.Skipf("Skipping known failure: %s", expected)
			}

			err := os.MkdirAll(filepath.Join(rootDir, "testdata", tt), os.ModePerm)
			require.NoError(t, err)

			result, err := engine.RunLanguageTest(context.Background(), &testingrpc.RunLanguageTestRequest{
				Token: prepare.Token,
				Test:  tt,
			})

			require.NoError(t, err)
			for _, msg := range result.Messages {
				t.Log(msg)
			}
			t.Logf("stdout: %s", result.Stdout)
			t.Logf("stderr: %s", result.Stderr)
			assert.True(t, result.Success)
		})
	}

	t.Cleanup(func() {
		close(cancel)
		assert.NoError(t, <-handle.Done)
	})
}

// expectedFailures maps the set of conformance tests we expect to fail to reasons they currently do so, so that we may
// skip them with an informative message until they are fixed.
var expectedFailures = map[string]string{
	"l1-builtin-secret":                      "unsupported config variable type: output(string)",
	"l1-builtin-require-pulumi-version":      "unsupported node type: dynamic",
	"l1-builtin-try":                         "unsupported config variable type: map(string)",
	"l1-config-types-primitive":              "unsupported operation type in expression lowering",
	"l1-config-types-object":                 "unsupported config variable type: map(int)",
	"l1-builtin-to-json":                     "unsupported config variable type: list(string)",
	"provider-replacement-trigger-component": "requires language providers directory support for conformance-component",
	"l3-for-resource":                        "unsupported expression type: for-expression",
	"policy-dryrun":                          "unsupported operation type in expression lowering",
	"provider-alias-component":               "requires language providers directory support for conformance-component",
	"l1-builtin-list":                        "unsupported config variable type: list(string)",
	"l3-splat":                               "unsupported expression type: splat-expression",
	"policy-simple":                          "unsupported operation type in expression lowering",
	"l1-builtin-object":                      "unsupported config variable type: map(string)",
	"l3-range":                               "unsupported config variable type: list(string)",
	"provider-ignore-changes-component":      "requires language providers directory support for conformance-component",
	"l3-for":                                 "unsupported config variable type: list(string)",
	"l1-config-secret":                       "unsupported config variable type: output(number)",
	"l1-builtin-can":                         "unsupported config variable type: map(string)",
	"l3-component-simple":                    "unsupported node type: object with output fields",
	"l1-proxy-index":                         "unsupported config variable type: object with union members",
	"provider-resource-component":            "requires language providers directory support for conformance-component",
	"l3-deferred-outputs":                    "unsupported node type: object with output fields",
}

// runTestingHost boots up a new instance of the language conformance test runner, `pulumi-test-language`, as well as a
// fake Pulumi engine for collecting logs. It returns the address of the fake engine and a connection to the test runner
// that can be used to manage a test suite run.
func runTestingHost(t *testing.T) (string, testingrpc.LanguageTestClient) {
	// We can't just go run the pulumi-test-language package because of
	// https://github.com/golang/go/issues/39172, so we build it to a temp file then run that.
	binary := t.TempDir() + "/pulumi-test-language"
	cmd := exec.Command("go", "build", "-o", binary, "github.com/pulumi/pulumi/pkg/v3/testing/pulumi-test-language") //nolint:gosec,lll
	output, err := cmd.CombinedOutput()
	t.Logf("build output: %s", output)
	require.NoError(t, err)

	cmd = exec.Command(binary)
	stdout, err := cmd.StdoutPipe()
	require.NoError(t, err)
	stderr, err := cmd.StderrPipe()
	require.NoError(t, err)
	stderrReader := bufio.NewReader(stderr)

	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		for {
			text, err := stderrReader.ReadString('\n')
			if err != nil {
				wg.Done()
				return
			}
			t.Logf("engine: %s", text)
		}
	}()

	err = cmd.Start()
	require.NoError(t, err)

	stdoutBytes, err := io.ReadAll(stdout)
	require.NoError(t, err)

	address := string(stdoutBytes)

	conn, err := grpc.NewClient(
		address,
		grpc.WithTransportCredentials(insecure.NewCredentials()),
		grpc.WithUnaryInterceptor(rpcutil.OpenTracingClientInterceptor()),
		grpc.WithStreamInterceptor(rpcutil.OpenTracingStreamClientInterceptor()),
		rpcutil.GrpcChannelOptions(),
	)
	require.NoError(t, err)

	client := testingrpc.NewLanguageTestClient(conn)

	t.Cleanup(func() {
		assert.NoError(t, cmd.Process.Kill())
		wg.Wait()
		// We expect this to error because we just killed it.
		contract.IgnoreError(cmd.Wait())
	})

	return address, client
}
