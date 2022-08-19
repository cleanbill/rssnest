package main

import (
	"github.com/bangbangsoftware/config"
	"github.com/bangbangsoftware/feeds"
	"log"
)

func Propagate(store feeds.Store, target Target, prices string, short Shortener, conf config.Settings) {
	log.Printf("\nPropagate the rss feeds results...")

	qty := conf.Propagate.QtyPerPage
	filename := conf.General.DataDir + conf.General.StoreName
	log.Printf("Propagate last %v results...", qty)

	newItems := store.GetLast(qty, filename)
	target.Send(newItems, prices)

	sendList := store.GetToMessage()
	for i := range sendList {
		msg := sendList[i].Item.Title
		url := sendList[i].Link
		target.Message(short, msg, url)
	}
}
