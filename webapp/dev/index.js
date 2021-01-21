import { SymcodeScanner, SymcodeConfig, AlphabetReaderParams } from "symcode";
import CONFIG from "./config";

const canvas = document.getElementById('frame');
const loadBuffer = document.getElementById('loadBuffer');
const loadBufferCtx = loadBuffer.getContext('2d');
const debugCanvas = document.getElementById('debug');
const ctx = canvas.getContext('2d');
const camera = document.getElementById('camera');
const cameraButton = document.getElementById('cameraButton');
const img = new Image();
const numTemplates = 4;

let debugging = true;
let finishScanning = false;

const scannerConfig = SymcodeConfig.from_json_string(JSON.stringify(CONFIG.SYMCODE_CONFIG));
const scanner = SymcodeScanner.from_config(scannerConfig);

const inputFrameSize = {
    width: 350,
    height: 350,
};
const fps = 60;

function loadingCompletes() {
    console.log("Template loading completes.");
    scanImageFromSource("assets/prototype_4/3.png");
}

const ERROR_COLOR = "color: #ff5050;";

function handleError(e) {
    console.log("%c" + e, ERROR_COLOR);
}

// Returns true if a Symcode is recognized and decoded
function scan() {
    try {
        let startTime = new Date();
        const result = scanner.scan();
        console.log("Scanning finishes in " + (new Date() - startTime) + " ms.");
        return result;
    } catch (e) {
        throw e;
    }
}

document.getElementById('generate').addEventListener('click', function (e) {
    let groundTruthCode = scanner.generate_symcode_to_canvas();
    console.log("Generated code: " + groundTruthCode);
    try {
        const result = scan();
        console.log("Recognition result: " + result);
        if (result.localeCompare(groundTruthCode) == 0) {
            console.log("%c Generated code is correctly recognized.", "color: #00ff00;");
        } else {
            console.log("%c Generated code is INCORRECTLY recognized.", ERROR_COLOR);
        }
    } catch (e) {
        handleError(e);
    }
});

document.getElementById('imageInput').addEventListener('change', function (e) { scanImageFromSource(this.files[0]) });

document.addEventListener('load', loadAlphabet());

function scanImageFromSource(source) {
    img.src = source instanceof File ? URL.createObjectURL(source) : source;
    img.onload = function () {
        canvas.width = img.naturalWidth;
        canvas.height = img.naturalHeight;

        debugCanvas.width = canvas.width;
        debugCanvas.height = canvas.height;

        ctx.clearRect(0, 0, canvas.width, canvas.height);
        ctx.drawImage(img, 0, 0);
        try {
            console.log("Recognition result: " + scan());
        } catch (e) {
            handleError(e);
        }
    };
}

function loadAllTemplates() {
    loadTemplateByIndex(1);
}

function loadTemplateByIndex(index) {
    if (index > numTemplates) {
        loadingCompletes();
        return;
    }
    const path = "assets/glyph_templates/" + index + ".jpg";
    img.src = path;
    img.onload = function () {
        loadBuffer.width = img.naturalWidth;
        loadBuffer.height = img.naturalHeight;

        loadBufferCtx.clearRect(0, 0, loadBuffer.width, loadBuffer.height);
        loadBufferCtx.drawImage(img, 0, 0);

        scanner.load_template_from_canvas_id('loadBuffer');

        loadTemplateByIndex(index + 1);
    };
}

function loadAlphabet() {
    const path = "assets/alphabet/alphabet2.jpg";
    const params = AlphabetReaderParams.from_json_string(JSON.stringify(CONFIG.ALPHABET_CONFIG));
    img.src = path;
    img.onload = function () {
        loadBuffer.width = img.naturalWidth;
        loadBuffer.height = img.naturalHeight;

        loadBufferCtx.clearRect(0, 0, loadBuffer.width, loadBuffer.height);
        loadBufferCtx.drawImage(img, 0, 0);

        scanner.load_alphabet_from_canvas_id('loadBuffer', params);

        loadingCompletes();
    };
}

const constraints = {
    video: { width: {min: 720}, height: {min: 720} },
};

function stopCamera() {
    const stream = camera.srcObject;
    stream.getTracks().forEach(function(track) {
        track.stop();
    });
    camera.srcObject = null;
}

cameraButton.onclick = function() {
    navigator.mediaDevices
        .getUserMedia(constraints)
        .then(handleGetCameraSuccess)
        .catch((e) => console.error(e));
}

function handleGetCameraSuccess(stream) {
    //camera.style.display = "block";
    camera.srcObject = stream;
    getCameraVideoDimensions()
        .then(({width, height}) => {
            startStreaming(width, height);
        });
}

function getCameraVideoDimensions() {
    return new Promise(function(resolve) {
        camera.addEventListener("loadedmetadata", function () {
            let width = this.videoWidth;
            let height = this.videoHeight;
            resolve({
                width: width,
                height: height,
            });
        }, false);
    });
}

function startStreaming(videoWidth, videoHeight) {
    console.log("Start streaming");
    console.log(videoWidth + " " + videoHeight);
    const sx = (videoWidth - inputFrameSize.width) / 2;
    const sy = (videoHeight - inputFrameSize.height) / 2;

    finishScanning = false;
    while (!finishScanning) {
        drawFrame(sx, sy);
        sleep(1/fps);
    }
}

function drawFrame(sx, sy) {
    canvas.width = inputFrameSize.width;
    canvas.height = inputFrameSize.height;
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.drawImage(camera, sx, sy, inputFrameSize.width, inputFrameSize.height,
                                        0, 0, canvas.width, canvas.height);
    try {
        console.log("Recognition result: " + scan());
        stopCamera();
        finishScanning = true;
    } catch (e) {
        handleError(e);
    }
}

function sleep(s) {
    const ms = s*1000;
    return new Promise(resolve => setTimeout(resolve, ms));
}