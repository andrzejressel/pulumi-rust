package rust

/*
#cgo LDFLAGS: ./../target/debug/libpulumi_rust.a
*/
import "C"
import (
	"bytes"
	"encoding/json"
	"fmt"

	"github.com/andrzejressel/pulumi-ast/codegen/ast"
	"github.com/hashicorp/hcl/v2"
	"github.com/pulumi/pulumi/pkg/v3/codegen/pcl"
	"github.com/pulumi/pulumi/pkg/v3/codegen/schema"
	"google.golang.org/protobuf/encoding/protojson"
	"google.golang.org/protobuf/proto"
)

func GeneratePackage(pkg *schema.Package, dir string) error {
	protobufPkg, err := ast.GenerateProtobufPackage(pkg)
	if err != nil {
		return fmt.Errorf("error generating protobuf package: %v", err)
	}

	obj, err := proto.Marshal(protobufPkg)
	if err != nil {
		return fmt.Errorf("error generating protobuf package: %v", err)
	}

	req := GeneratePackageRequest{
		protobuf:  obj,
		directory: dir,
	}

	result := G2RCallImpl{}.generate_package(&req)
	if result.error != "" {
		return fmt.Errorf("error generating rust package: %s", result.error)
	}

	return nil
}

func GenerateProgram(pkg *pcl.Program) (map[string][]byte, hcl.Diagnostics, error) {
	protobufProgram, err := ast.GenerateProtobuf(pkg)
	if err != nil {
		return nil, nil, fmt.Errorf("error generating protobuf program: %v", err)
	}

	obj, err := proto.Marshal(protobufProgram)
	if err != nil {
		return nil, nil, fmt.Errorf("error generating protobuf package: %v", err)
	}

	req := GenerateProgramRequest{
		protobuf: obj,
	}

	result := G2RCallImpl{}.generate_program(&req)
	if result.error != "" {
		return nil, nil, fmt.Errorf("error generating rust program: %s", result.error)
	}

	emptyMap := make(map[string][]byte)

	for _, content := range result.files_content {
		emptyMap[content.path] = content.content
	}

	return emptyMap, nil, nil

}

func GenerateProject(pkg *pcl.Program, dir string) ([]byte, []byte, error) {
	protobufProgram, err := ast.GenerateProtobuf(pkg)
	if err != nil {
		return nil, nil, fmt.Errorf("error generating protobuf program: %v", err)
	}

	obj, err := proto.Marshal(protobufProgram)
	if err != nil {
		return nil, nil, fmt.Errorf("error generating protobuf package: %v", err)
	}

	protobufJSON, err := protojson.MarshalOptions{
		UseProtoNames: true,
	}.Marshal(protobufProgram)
	if err != nil {
		return nil, nil, fmt.Errorf("error generating protobuf JSON: %v", err)
	}
	protobufJSON, err = normalizeJSON(protobufJSON)
	if err != nil {
		return nil, nil, fmt.Errorf("error normalizing protobuf JSON: %v", err)
	}

	req := GenerateProjectRequest{
		protobuf:  obj,
		directory: dir,
	}

	result := G2RCallImpl{}.generate_project(&req)
	if result.error != "" {
		return nil, nil, fmt.Errorf("error generating rust project: %s", result.error)
	}

	return obj, protobufJSON, nil
}

func normalizeJSON(input []byte) ([]byte, error) {
	var output bytes.Buffer
	if err := json.Indent(&output, input, "", "  "); err != nil {
		return nil, err
	}
	return output.Bytes(), nil
}
