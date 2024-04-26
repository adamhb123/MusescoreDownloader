const timeout = (ms) => { return new Promise(res => setTimeout(res, ms)) };


const waitForImg = (page) => new Promise(async res => {
    while (true) {
        try {
            console.log("Searching for image...");
            let src = page.getElementsByTagName("img")[0].getAttribute("src");
            if (src) {
                console.log("Found image src!");
                return res(src);
            } else {
                console.log("Unable to find image src...retrying")
                await timeout(1000);
            }
        } catch {
            console.log("Unable to find img...retrying")
            await timeout(1000);
        }
        await timeout(500);
    }
});
const downloadURI = async (uri, name) => {
    return new Promise(res => {
        var link = document.createElement("a");
        link.download = name;
        link.href = uri;
        console.log(`HREF: ${uri}`);
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
        delete link;
        res();
    });
}
const downloadPageImgs = async () => {
    const cname = document.getElementById("jmuse-scroller-component").children[0].getAttribute("class");
    const pages = document.getElementsByClassName(cname);
    console.log(pages);
    for (let i = 0; i < pages.length; i++) {
        console.log(`Downloading page ${i+1}/${pages.length}`);
        let page = pages[i]
        page.scrollIntoView();
        await waitForImg(page);
        let uri = page.getElementsByTagName("img")[0].getAttribute("src");
        await downloadURI(uri, `score_${i}.png`);
        console.log(page);
        console.log(`Finished downloading page ${i+1}/${pages.length}`);
    }
}

(async () => {
    console.log("Hello from Musescore Downloader!");
    const title = document.getElementById("aside-container-unique").getElementsByTagName("h1")[0].innerText;
    console.log(`Downloading score: "${title}"`);
    await downloadPageImgs();
    console.log(`Finished downloading score: "${title}"`);
    // await printJS({
    //     printable: img_dataurls,
    //     type: 'pdf'
    // });
})();


