package feeds

import (
	"encoding/json"
	"io/ioutil"
	"log"
	"os"

	"github.com/bangbangsoftware/config"
)

type Misc struct {
	User      string
	CreatedOn string
}

type RssFeed struct {
	Name string
	Desc string
	Url  string
	Date string
	Dir  string
}

type Casts struct {
	Misc  Misc
	Feeds []RssFeed
}

func loadRSSList(conf config.Settings) Casts {
	log.Printf("\n")
	log.Printf("\n")
	log.Printf("\n")
	log.Printf("\n*****************************************************************\n")
	log.Printf("loading rss list from: %s \n", conf.General.Feedfile)
	castsFile, e2 := ioutil.ReadFile(conf.General.Feedfile)
	if e2 != nil {
		log.Printf("RSS list file error: %v\n", e2)
		os.Exit(1)
	}
	var cs Casts
	json.Unmarshal(castsFile, &cs)
	log.Printf("%s feed created on %s \n", cs.Misc.User, cs.Misc.CreatedOn)
	return cs
}

// Maybe this just take a harvester which contains the source and the store and the processing logic
func StoreNewContent(store Store, source Source) {
	conf := config.GetConfig()
	var cs = loadRSSList(conf)
	var total = len(cs.Feeds)
	log.Printf("%v RSS feeds found \n", total)
	for i := 0; i < total; i++ {
		item := cs.Feeds[i]
		name := item.Name
		log.Printf("\n")
		log.Printf("\n=================================================================\n")
		log.Printf("[%v/%v] %s (%s) \n", i, total, name, item.Date)
		Leach(item, store, source, 1)
	}
}
