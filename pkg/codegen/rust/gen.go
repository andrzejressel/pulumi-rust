package rust

/*
#cgo LDFLAGS: ./../target/debug/libpulumi_rust.a
*/
import "C"
import (
	"fmt"

	"github.com/andrzejressel/pulumi-ast/codegen/ast"
	"github.com/hashicorp/hcl/v2"
	"github.com/pulumi/pulumi/pkg/v3/codegen/pcl"
	"github.com/pulumi/pulumi/pkg/v3/codegen/schema"
	"google.golang.org/protobuf/proto"
)

func GenerateJSONPackage(pkg *schema.Package, dir string) error {
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

	_ = G2RCallImpl{}.generate_package(&req)

	//data, err := proto.Marshal(req)
	//if err != nil {
	//	return nil, nil, fmt.Errorf("error generating protobuf package: %v", err)
	//}

	//panic("TEST")
	return nil
}

func GenerateJSONProgram(pkg *pcl.Program) (map[string][]byte, hcl.Diagnostics, error) {
	//panic("TEST")

	empty_map := make(map[string][]byte)

	return empty_map, nil, nil

}
