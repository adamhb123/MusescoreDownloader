const timeout = (ms) => new Promise(res => setTimeout(res, ms));
const wait_for_img = (page) => new Promise(async res => {
    while (true) {
        try {
            let src = page.getElementsByTagName("img")[0].getAttribute("src");
            if (src) {
                return res(src);
            }
        } catch {
            console.debug("Unable to find img src...")
            await timeout(1000);
        }
    }
});
const downloadURI = async (uri, name) => new Promise(res => {
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
const get_page_imgs = async () => {
    const cname = document.getElementById("jmuse-scroller-component").children[0].getAttribute("class");
    const pages = document.getElementsByClassName(cname);
    const dataURLs = [];
    console.log(pages);
    for (let i in pages) {
        let page = pages[i]
        page.scrollIntoView();
        await wait_for_img(page);
        // let uri = page.getElementsByTagName("img")[0].getAttribute("src");
        dataURLs.push(toDataURL(page.getElementsByTagName("img")[0]['dataURL']));
        //await downloadURI(uri, `score_${i}.png`);
        console.log(page);
    }
    return dataURLs;
}

const getBase64StringFromDataURL = (dataURL) => dataURL.replace('data:', '').replace(/^.+,/, '');

const toDataURL = (image) => {
    const canvas = document.createElement('canvas');
    canvas.width = image.naturalWidth;
    canvas.height = image.naturalHeight;
    canvas.getContext('2d').drawImage(image, 0, 0);
    const dataURL = canvas.toDataURL();
    const base64 = getBase64StringFromDataURL(dataURL);
    return {dataURL, base64};
};

(async () => { // import print.js
    const src = chrome.runtime.getURL("scripts/print.min.js");
    console.log("SRC: " + src);
    await import(src);
})().then(async () => {
    console.log("Hello from Musescore Downloader!");
    console.log("GETTING");
    let img_dataurls = await get_page_imgs();
    console.log("GOT");
    console.debug(img_dataurls);
    // await printJS({
    //     printable: img_dataurls,
    //     type: 'pdf'
    // });
});


