package main

import (
	"flag"
	"fmt"
	//"io"
	//"net/http"
	//"os"
	//"github.com/schollz/progressbar/v3"
)

type FileDownload struct {
	name string
	url  string
}

func main() {
	flags := flag.CommandLine.Args()
	for i := 0; i < len(flags); i++ {
		fmt.Println(flags[i])
	}
}
