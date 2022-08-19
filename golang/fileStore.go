package main

import (
	"encoding/json"
	"github.com/bangbangsoftware/feeds"
	"io/ioutil"
	"log"
	"os"
	"sort"
	"strings"
)

var GotAlready map[string]feeds.RssResult
var toMessage []feeds.RssResult

//var newStore []feeds.RssResult
var Perm os.FileMode = 0777

type FileStore struct {
}

func (fs FileStore) ClearMessages() {
	toMessage = toMessage[:0]
}

func (fs FileStore) loadItems(filename string) map[string]feeds.RssResult {
	log.Printf("About to load %v\n", filename)
	file, e := ioutil.ReadFile(filename)
	var data []byte
	if GotAlready == nil {
		GotAlready = make(map[string]feeds.RssResult)
	}

	if e != nil {
		if strings.Contains(e.Error(), "no such file") {
			log.Printf("No %v file, so creating one\n", filename)
			ioutil.WriteFile(filename, data, Perm)
		} else {
			log.Printf("File error: %v\n", e)
			os.Exit(1)
		}
	}
	json.Unmarshal(file, &GotAlready)
	log.Printf("already have list is: %v \n", len(GotAlready))
	return GotAlready
}

type ByDate []feeds.RssResult

func (a ByDate) Len() int           { return len(a) }
func (a ByDate) Swap(i, j int)      { a[i], a[j] = a[j], a[i] }
func (a ByDate) Less(i, j int) bool { return a[i].Date.After(a[j].Date) }

func (fs FileStore) GetLast(i int, filename string) []feeds.RssResult {
	if len(GotAlready) == 0 {
		GotAlready = fs.loadItems(filename)
	} else {
		log.Printf("No loading needed %v list is: %v \n", filename, len(GotAlready))
	}
	var n []feeds.RssResult
	for _, v := range GotAlready {
		n = append(n, v)
	}
	if len(n) < i {
		i = len(n)
	}
	sort.Sort(ByDate(n))
	var last []feeds.RssResult
	for c := 0; c < i; c++ {
		last = append(last, n[c])
	}
	return last
}

func (fs FileStore) GetToMessage() []feeds.RssResult {
	return toMessage
}

func (fs FileStore) AlreadyHave(itemLink string, filename string) bool {
	if len(GotAlready) == 0 {
		GotAlready = fs.loadItems(filename)
	} else {
		log.Printf("No loading needed %v list is: %v \n", filename, len(GotAlready))
	}
	if _, ok := GotAlready[itemLink]; ok {
		return true
	}
	return false
}

func (fs FileStore) Save(rssResult feeds.RssResult, link string, filename string) {
	if len(rssResult.Item.Title) == 0 {
		log.Printf("Skipping saving as this result has no title")
		return
	}
	if len(GotAlready) == 0 {
		GotAlready = fs.loadItems(filename)
	} else {
		log.Printf("Adding to the %v items done with %v \n", len(GotAlready), link)
	}
	GotAlready[link] = rssResult
	if rssResult.Failed {
		log.Printf("No message as the rss failed")
	} else {
		rssResult.Link = link
		toMessage = append(toMessage, rssResult)
	}
	jsave, _ := json.Marshal(GotAlready)
	var data []byte = jsave
	ioutil.WriteFile(filename, data, Perm)
}
