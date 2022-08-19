package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"os"

	"github.com/ChimeraCoder/anaconda"
	"github.com/bangbangsoftware/config"
	"github.com/bangbangsoftware/feeds"
	"github.com/dutchcoders/goftp"
)

type Target interface {
	Send(newItems []feeds.RssResult, prices string)
	Message(util Shortener, name string, url string)
}

type FtpTarget struct {
}

func (s FtpTarget) Send(newItems []feeds.RssResult, prices string) {
	conf := config.GetConfig()
	var perm os.FileMode = 0777
	var err error
	var ftp *goftp.FTP

	log.Printf("...saving newData (%v) file to ftp to %v\n", len(newItems), conf.Propagate.Ftp.Url)
	var newData = []byte("var data = \n")
	jsave, _ := json.Marshal(newItems)
	for _, e := range jsave {
		newData = append(newData, e)
	}
	ioutil.WriteFile(conf.General.DataDir+"newData.json", newData, perm)
	var file *os.File
	if file, err = os.Open(conf.General.DataDir + "newData.json"); err != nil {
		panic(err)
	}
	defer file.Close()
	addr := fmt.Sprintf("%v:%v", conf.Propagate.Ftp.Url, conf.Propagate.Ftp.Port)
	if ftp, err = goftp.Connect(addr); err != nil {
		panic(err)
	}
	defer ftp.Close()
	if err = ftp.Login(conf.Propagate.Ftp.Usr, conf.Propagate.Ftp.Pw); err != nil {
		panic(err)
	}
// This is failing for some reason????    
// One line curl seems to do it....    curl -T newData.json ftp://blarblar.net --user fred:donttell 
//	if err := ftp.Stor("newData.json", file); err != nil {
//		panic(err)
//	}

	log.Printf("saving prices file to ftp to webserver\n\n\n")
	var data3 = []byte("var prices = "+ prices+"\n")
//	jsave2, _ := json.Marshal(prices)
//	for _, e := range jsave2 {
//		data3 = append(data3, e)
//	}
	ioutil.WriteFile(conf.General.DataDir+"prices.json", data3, perm)
	var pricesFile *os.File
	if pricesFile, err = os.Open(conf.General.DataDir + "prices.json"); err != nil {
		panic(err)
	}
	defer pricesFile.Close()
// This is failing for some reason????    
// One line curl seems to do it....    curl -T price.json ftp://blarblar.net --user fred:donttell 
//	if err := ftp.Stor("prices.json", pricesFile); err != nil {
//		panic(err)
//	}
}

func (s FtpTarget) Message(util Shortener, msg string, url string) {
	conf := config.GetConfig()
	anaconda.SetConsumerKey(conf.Propagate.Tweet.ConsumerKey)
	anaconda.SetConsumerSecret(conf.Propagate.Tweet.ConsumerSecret)
	api := anaconda.NewTwitterApi(conf.Propagate.Tweet.AccessTokenKey, conf.Propagate.Tweet.AccessTokenSecret)
	short := util.Shorten(url, conf.Propagate.Apikey)
	link := url
	if short.Err == nil {
		log.Printf("%v shortened to %v", url, short.Id)
		link = short.Id
	} else {
		log.Printf("Failed shortening")
		log.Printf("%v", short.Err)
	}

	full := link + " " + msg
	if len(full) > 140 {
		full = full[0:137] + "..."
	}
	log.Printf("Full message to tweet is '%v'\n", full)
	api.PostTweet(full, nil) // "https://twitter.com/rssnest")
}
