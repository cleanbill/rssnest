// Redo another page with the error?

function f(i) {
    if (i > 9) {
        return "" + i;
    }
    return "0" + i;
}

function t(w) {
    var s = w + "";
    var i = s.indexOf(".");
    return s.substring(0, i + 3);
}

function p() {
    console.log("prices");
    console.log(prices);
    if (prices.rates != null && prices.rates.spot != null) {
        var golds = prices.rates.spot.filter(rate => (rate.baseCurrency === 'Gold' && rate.quoteCurrency === 'GBP'))
        var gold = golds[0].bid
        console.log(typeof gold) 
        var ag = prices.rates.spot.filter(rate => (rate.baseCurrency === 'Silver' && rate.quoteCurrency === 'GBP'))[0].bid
        if (ag && gold) {
            return `${gold.toFixed(4)}/gg, ${ag.toFixed(4)}/oz (28.0024/gg - 16.8322/oz)`;
        }
    }
    return "?/gg, ?/oz (28,17)";
}

function replaceTag(filt, id, includeFail) {

    var gold = "?/gg, ?/oz (28,17)";
    try {
        gold = p();
    } catch (err) {
        console.log(err);
    }
    var cnt = 1;
    var items = data.filter(function(post) {
        return filt(post);
    }).sort(function(p1, p2) {
        var p1d = new Date(p1.Date);
        var p2d = new Date(p2.Date);
        if (p1d > p2d) {
            return -1;
        } else if (p2d > p1d) {
            return 1;
        }
        return 0;
    }).map(function(post) {
        var ad = new Date(post.Date);
        console.log("posts...");
        console.log(post);

        var stDate = `${f(ad.getDate())}-${f(ad.getMonth()+1)}-${ad.getFullYear()} ${ad.getHours()}:${f(ad.getMinutes())}`;
        var head = `<div class="demo-card-wide mdl-card mdl-shadow--2dp">
              <div class="mdl-card__title">
                <h5 class="mdl-card__title-text">${cnt}. ${post.Name}:</h5></br>
                <h1 title="${post.Name}" class="mdl-card__title-text"><b>${post.Item.Title}</b></h1></br>
              </div>
                <h8>${stDate} - ${gold}</h8></br>`;
        if (includeFail) {
            head = `${head}<h3> ${post.FailReason}</h3>`;
        }
        var body = `
              <div align="left" class="mdl-card__supporting-text">${post.Item.Description}</div>
              <div class="mdl-card__actions mdl-card--border">
                <a title='${post.Message}' target='_blank' href='${post.Item.Link}' 
                   class="mdl-button mdl-button--colored mdl-js-button mdl-js-ripple-effect">Link</a>
              </div>
            </div>
            </br>`;
        cnt = cnt + 1;
        return `${head}${body}`;
    });
    var allHTML = "";
    items.forEach(function(l) {
        allHTML = `${allHTML}${l}`;
    });
    if (allHTML.length === 0) {
        console.log("No content?!");
        allHTML = "No, nothing, nada... nope";
    }
    var elem = document.querySelector(id);
    if (elem == null) {
        console.log("Cannot find element with '"+id+"'");
    } else {        
        elem.innerHTML = allHTML;
    }
}

let state = "compiled and loaded"
console.log(`dynamically loaded ${state}`)
console.log(data);
var notFailed = function(post) {
    return !post.Failed && post.Item.Title.length > 0;
};
replaceTag(notFailed, '#list', false);
var failed = function(post) {
    return post.Failed;
};
replaceTag(failed, '#error', true);



