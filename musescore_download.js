const timeout = (ms) => new Promise(res => setTimeout(res, ms));
const downloadURI = (uri, name) => {
    var link = document.createElement("a");
    link.download = name;
    link.href = uri;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    delete link;
  }
const get_page_imgs = async (cname) => {
    const pages = document.getElementsByClassName(cname);
    console.log(pages);
    for (let i in pages) {
            let page = pages[i]
            page.scrollIntoView();
            await timeout(2000);
            let uri = page.getElementsByTagName("img")[0];
            downloadURI(uri, `score_${i}.png`);
            console.log(page);
        
    }
}
