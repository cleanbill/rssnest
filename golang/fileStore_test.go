package main

import (
	"fmt"
	"github.com/bangbangsoftware/feeds"
	"os"
	"testing"
	"time"
)

var filename = "testStore.json"

func tearDown() {
	os.Remove(filename)
}

func TestIgnoringBlanks(t *testing.T) {
	defer tearDown()

	var store = new(FileStore)
	var data = new(feeds.RssResult)
	var link = "http://one"
	store.Save(*data, link, filename)
	var res = store.GetLast(10, filename)
	if len(res) > 0 {
		t.Error("Should skip any results with no titles")
	}
	var mess = store.GetToMessage()
	if len(mess) > 0 {
		t.Error("Should have no new result to message")
	}

	fmt.Println("======================================")
}

func TestSaveTitled(t *testing.T) {
	defer tearDown()

	var store = new(FileStore)
	var data = new(feeds.RssResult)
	data.Item.Title = "Golang content"
	var link = "http://one"
	store.Save(*data, link, filename)
	var res = store.GetLast(10, filename)
	if len(res) != 1 {
		t.Error("Should have one result with a title")
	}
	if res[0].Item.Title != data.Item.Title {
		fmt.Println(res[0])
		t.Error("Expected '", res[0].Item.Title, "' to equal '"+data.Item.Title+"'")
	}
	var mess = store.GetToMessage()
	if len(mess) != 1 {
		t.Error("Should have one new result to message")
	}
	fmt.Println("======================================")
}

func TestPutInDateOrder(t *testing.T) {
	defer tearDown()

	var store = new(FileStore)
	store.ClearMessages()
	var mess = store.GetToMessage()
	if len(mess) != 0 {
		fmt.Println(mess)
		t.Error("Should have no new result to message but has", len(mess))
	}

	var data = new(feeds.RssResult)
	data.Item.Title = "Golang content two hours ahead"
	data.Date = time.Now().Add(time.Hour)
	var link = "http://one"
	store.Save(*data, link, filename)
	mess = store.GetToMessage()
	if len(mess) != 1 {
		t.Error("Should have one new result to message but has", len(mess))
	}

	var data2 = new(feeds.RssResult)
	data2.Item.Title = "Golang content one hour ahead"
	data2.Date = time.Now().Add(time.Hour).Add(time.Hour)
	var link2 = "http://two"
	store.Save(*data2, link2, filename)
	mess = store.GetToMessage()
	if len(mess) != 2 {
		t.Error("Should have two new result to message but has", len(mess))
	}
	var gotit = store.AlreadyHave(link2, filename)
	if !gotit {
		t.Error(link2, " has been stored but isn't")
	}

	var data3 = new(feeds.RssResult)
	data3.Item.Title = "Golang content now"
	data3.Date = time.Now()
	var link3 = "http://three"
	store.Save(*data3, link3, filename)
	mess = store.GetToMessage()
	if len(mess) != 3 {
		t.Error("Should have three new result to message but has", len(mess))
	}

	var data4 = new(feeds.RssResult)
	data4.Item.Title = "Golang content one hour behind"
	data4.Date = time.Now().Add(-time.Hour)
	data4.Failed = true
	var link4 = "http://four"
	store.Save(*data4, link4, filename)
	mess = store.GetToMessage()
	if len(mess) != 3 {
		t.Error("Should still have three new result to message but has", len(mess))
	}

	var data5 = new(feeds.RssResult)
	data5.Item.Title = "Golang content two hours behind"
	data5.Date = time.Now().Add(-time.Hour).Add(-time.Hour)
	data5.Failed = false
	var link5 = "http://five.rss"
	store.Save(*data5, link5, filename)
	mess = store.GetToMessage()
	if len(mess) != 4 {
		t.Error("Should have four new result to message but has", len(mess))
	}

	var res = store.GetLast(10, filename)
	if len(res) != 5 {
		t.Error("Should have five results with a title")
	}
	var check = func(no int, title string) {
		if res[no].Item.Title != title {
			fmt.Println(res[no])
			t.Error("Expected '" + res[no].Item.Title + "' to equal '" + title + "'")
		}
	}
	check(0, data2.Item.Title)
	check(1, data.Item.Title)
	check(2, data3.Item.Title)
	check(3, data4.Item.Title)
	check(4, data5.Item.Title)
	gotit = store.AlreadyHave(link, filename)
	if !gotit {
		t.Error(link, " has been stored but isn't")
	}
	gotit = store.AlreadyHave(link3, filename)
	if !gotit {
		t.Error(link3, " has been stored but isn't")
	}
	gotit = store.AlreadyHave(link4, filename)
	if !gotit {
		t.Error(link4, " has been stored but isn't")
	}
	gotit = store.AlreadyHave(link5, filename)
	if !gotit {
		t.Error(link5, " has been stored but isn't")
	}
	gotit = store.AlreadyHave("no not yet rss feed link result", filename)
	if gotit {
		t.Error("This has not been stored but is")
	}

	res = store.GetLast(4, filename)
	if len(res) != 4 {
		t.Error("Should be restricted to four results with a title")
	}

}
