package feeds

import (
	//	"bytes"
	"fmt"
	"io"
	"log"
	"os"
	"strings"
	"time"

	"github.com/bangbangsoftware/config"
)

func Leach(feed RssFeed, store Store, src Source, howmany int) []RssResult {
	var items []RssResult
	rss := src.GetRSS(feed.Url)
	if rss == nil {
		log.Printf("No feed, all is done here\n")
		return items
	}

	log.Printf("Title : %s\n", rss.Channel.Title)
	log.Printf("Description : %s\n", rss.Channel.Description)
	log.Printf("Link : %s\n", rss.Channel.Link)

	total := len(rss.Channel.Items)

	log.Printf("Total items : %v\n", total)
	if total == 0 {
		log.Printf("No items, all is done here\n")
		return items
	}
	for i := 0; i < howmany; i++ {
		if i < len(rss.Channel.Items) {
			var rssResult RssResult
			rssResult.Name = feed.Name
			rssResult.Date = time.Now()
			rssResult.Item = rss.Channel.Items[i]
			result := checkAndGet(rssResult, i, store, src, feed.Dir, rss.Channel.Title)
			if result.AlreadyHave {
				log.Printf("got it already....")
			} else {
				log.Printf("Item found\n")
				items = append(items, result)
			}
		}
	}
	return items
}

func checkAndGet(r RssResult, i int, hold Store, traffic Source, dir string, name string) RssResult {
	conf := config.GetConfig()
	filename := conf.General.DataDir + conf.General.StoreName
	var link = r.Item.Link
	if len(r.Item.Enclosure.Url) > 0 {
		log.Printf("using enclosure url")
		link = r.Item.Enclosure.Url
	}
	log.Printf("[%d] %s link is: %s\n", i, r.Item.Title, link)
	if len(link) == 0 {
		log.Printf("[%d] NO LINK!!? \n", i, r.Item.Title, link)
		r.Failed = true
		r.FailReason = "No Link?"
		return r
	}

	if hold.AlreadyHave(link, filename) {
		log.Printf("Already have %s\n", link)
		r.AlreadyHave = true
		return r
	}
	log.Printf("NEW CONTENT: %v\n", link)

	response, err := traffic.Get(link)
	if err != nil {
		log.Printf("Link (%v) error: %v\n", link, err)
		r.Failed = true
		r.FailReason = "Link error"
		return r
	}

	urlname := getName(dir, link)
	log.Printf("urlname %v \n", urlname)
	defer response.Body.Close()
	log.Printf("response (%v) \n", response.Header)
	t := response.Header.Get("Content-Type")
	result := processContent(t, response.Body, urlname, traffic, r)
	result.Name = name
	hold.Save(result, link, filename)
	return result
}

func processContent(t string, body io.ReadCloser, name string, traffic Source, r RssResult) RssResult {
	defer body.Close()
	log.Printf("%s which is %s content type \n", name, t)
	if strings.Contains(t, "text/html") || strings.Contains(t, "text/xml") {
		r.Message = "Rss content type is text, nothing to download"
		log.Println(r.Message + "\n")
		return r
	}

	if !strings.Contains(name, ".") {
		name = name + ".mp3"
		r.Message = "Rss content had no extention, defaulted to mp3"
		log.Println(r.Message + "\n")
	}

	out, err := os.Create(name)
	defer out.Close()
	if err != nil {
		r.Failed = true
		r.FailReason = "Error creating file for content"
		log.Println(r.FailReason + "\n")
		return r
	}

	n, err := io.Copy(out, body)
	if err != nil {
		r.Failed = true
		r.FailReason = "Error copying content to file"
		log.Println(r.FailReason + "\n")
		return r
	}
	log.Println("Downloaded ", name, ", which was ", n, " bytes")
	out = nil

	r.Message = fmt.Sprintf("%v Downloaded %v bytes", r.Message, n)
	return r
}

func getName(subdir string, link string) string {
	conf := config.GetConfig()
	audioDir := conf.General.AudioDir
	visualDir := conf.General.VisualDir
	var bits = strings.Split(link, "/")
	t := time.Now().Format("200615040501") + "."
	var name = t + bits[len(bits)-1]
	name = strings.Split(name, "?")[0]
	var dir = audioDir + subdir + "/"
	if strings.HasSuffix(name, "mp4") || strings.HasSuffix(name, "mv4") {
		log.Println("Visual content\n")
		dir = visualDir + subdir
	}
	return dir + name
}
