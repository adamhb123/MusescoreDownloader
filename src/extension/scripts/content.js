const MSD_COMPANION_ADDR = "127.0.0.1:45542";
const USER_DOWNLOAD_DIR = "C:\\Users\\swat8\\Downloads"
const EXT_EXTRACTOR_REGEX = /.*score_\d*.*.(png|svg)/;

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
                console.log("Unable to find image src...retrying");
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

const getDownloads = async (args) => {
    // Args MUST be an array of values
    const response = await chrome.runtime.sendMessage({command: "get-downloads", args: args ?? [{}]});
    console.log(response);
    return response;
}

const waitForDownloadsFinished = async () => {
    await timeout(100);
    console.log("Waiting for downloads to finish...");
    let downloads = (await getDownloads()).filter((dl) => dl.state == "in_progress");
    console.log("Downloads in-progress: ");
    console.log(downloads);
    if (downloads.length == 0) return;
    await waitForDownloadsFinished();
}

const waitForPageDownload = async (page_idx) => {
    let regex =  `score_${page_idx}.*`;
    console.log(`Using regex: ${regex}`);
    dls = await getDownloads([{filenameRegex: regex}])
    console.log("Last page search (no filter):");
    console.log(dls);
    dls = dls.filter((e) => e.state == "complete");
    console.log("Last page search (filter):");
    console.log(dls);
    if (dls.length == 0) await waitForPageDownload(page_idx);
}

const downloadPageImgs = async () => {
    console.log("Downloading page imgs...")
    const cname = document.getElementById("jmuse-scroller-component").children[0].getAttribute("class");
    const pages = document.getElementsByClassName(cname);
    let paths = [];
    console.log(pages);
    for (let i = 0; i < pages.length; i++) {
        console.log(pages[i]);
        let page = pages[i]
        page.scrollIntoView();
        await waitForImg(page);
        let uri = page.getElementsByTagName("img")[0].getAttribute("src");
        let ext = uri.match(EXT_EXTRACTOR_REGEX)[1] ?? "png"; // Should be "png" or "svg"
        let file_name = `score_${i}.${ext}`;
        console.log(`EXTENSION:  ${ext}`);
        paths.push(`${USER_DOWNLOAD_DIR}\\${file_name}`);
        console.log(`Downloading page ${i + 1}/${pages.length}`);
        await downloadURI(uri, file_name);
        await waitForDownloadsFinished();
    }
    await waitForPageDownload(pages.length-1);
    return paths;
}

(async () => {
    console.log("Hello from Musescore Downloader!");
    console.log("Requesting clean-up of score files in download directory from msd-companion...");
    console.log(await fetch(`${MSD_COMPANION_ADDR}/clean-scores`));
    const title = document.getElementById("aside-container-unique").getElementsByTagName("h1")[0].innerText;
    console.log(`Downloading score: "${title}"`);
    let img_paths = await downloadPageImgs();
    console.log(`Finished downloading score pngs: "${title}"`);
    console.log(`Paths: "${img_paths}"`);
    let req_url = `http://${MSD_COMPANION_ADDR}/msd?fname=${title}&paths=${img_paths.join(',')}&output_dir=${USER_DOWNLOAD_DIR}`;
    console.log(`REQ_URL: ${req_url}`);
    await downloadURI(req_url, `${title}.pdf`);
    // await printJS({
    //     printable: img_dataurls,
    //     type: 'pdf'
    // });
})();


