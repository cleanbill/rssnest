// rssnest project
package main

import (
	"flag"
	"fmt"
	"github.com/bangbangsoftware/config"
	"github.com/bangbangsoftware/feeds"
	"log"
	"net/http"
	_ "net/http/pprof"
	"os"
)

var term bool = false

func setup() {
	//Switches
	memoryPort := flag.Int("mem", -1, "Turn memory listener on this port")
	configFile := flag.String("conf", "./conf.json", "The path to the configuration file")
	sendAssets := flag.Bool("ftp", false, "FTP the assets to the webserver")
	term = *flag.Bool("term", false, "Just log to the terminal")
	flag.Parse()

	// memory
	if *memoryPort > -1 {
		go func() {
			var listen = fmt.Sprintf("localhost:%v", memoryPort)
			log.Println(http.ListenAndServe(listen, nil))
		}()
	}

	// Config
	log.Printf("loading config from: %s \n", *configFile)
	config.LoadConfig(*configFile)

	// Ftp Assets
	if *sendAssets {
		log.Printf("Ftping assets to webserver")
		//ftpAssets(config)
	}
	return
}

func main() {

	setup()

	// Logging
	if term {
		log.Printf("logging to terminal \n")
	} else {
		log.Printf("logging to rssnest.log \n")
		f, err := os.OpenFile("rssnest.log", os.O_RDWR|os.O_CREATE|os.O_APPEND, 0666)
		if err != nil {
			log.Fatalf("error opening file: %v", err)
		}
		defer f.Close()
		log.SetOutput(f)
	}

	store := new(FileStore)
	source := new(HttpSource)
	feeds.StoreNewContent(store, source)

	target := new(FtpTarget)
	short := new(GoogleShort)
	conf := config.GetConfig()
	prices := feeds.GetPrices()
	Propagate(store, target, prices, short, conf)
}
