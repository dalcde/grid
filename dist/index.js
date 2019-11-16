'use strict';

import init, { dot, grid } from './grid.js';

init();

function download(filename, data) {
    if (!Array.isArray(data)) {
        data = [data];
    }
    let element = document.createElement('a');

    element.href = URL.createObjectURL(new Blob(data, {type : "application/pdf"}));
    element.download = filename;
    element.rel = 'noopener';
    element.dispatchEvent(new MouseEvent('click'));
    setTimeout(() => URL.revokeObjectURL(element.href), 6E4);
}
function hexToRgb (hex) {
    const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
    return [
        parseInt(result[1], 16),
        parseInt(result[2], 16),
        parseInt(result[3], 16)
    ];
};

const DIMENSIONS = {
    "A4": [11.7, 8.27],
    "A3": [16.5, 11.7],
    "Letter": [11, 8.5],
    "Tabloid": [17, 11]
};

let form = document.getElementById("form")

let updateColor = () => {
    let color = form["color"];
    color.parentElement.style.backgroundColor = color.value;
    const [r, g, b] = hexToRgb(color.value);

    // This is the Y value in YIQ
    const y = r * 0.299 + g * 0.587 + b * 0.114;
    color.parentElement.style.color = y > 180 ? 'black' : 'white';
    color.parentElement.style.borderColor = `rgb(${r - y/6}, ${g - y/6}, ${b - y/6})`;
}
form["color"].addEventListener("change", updateColor);
updateColor();

let updateNumXY = () => {
    form["num_x"].value = Math.floor((form["width"].value - 2 * form["margin"].value) / form["d"].value + 0.02);
    form["num_y"].value = Math.floor((form["height"].value - 2 * form["margin"].value) / form["d"].value + 0.02);
};

let updatePaper = () => {
    const type = form["paper"].value;
    let w = document.getElementById("width");
    let h = document.getElementById("height");
    if (type == "Custom") {
        form["width"].disabled = false;
        form["height"].disabled = false;
    } else {
        const [width, height] = DIMENSIONS[type];
        form["width"].value = width;
        form["height"].value = height;

        form["width"].disabled = true;
        form["height"].disabled = true;
    }
    updateNumXY();
}
form["paper"].addEventListener("change", updatePaper);
updatePaper();

form["margin"].addEventListener("change", updateNumXY);
form["width"].addEventListener("change", updateNumXY);
form["height"].addEventListener("change", updateNumXY);
form["d"].addEventListener("change", updateNumXY);

form["num_x"].addEventListener("change", () => {
    form["d"].value = ((form["width"].value - 2 * form["margin"].value) / form["num_x"].value).toFixed(5);
    updateNumXY();
});
form["num_y"].addEventListener("change", () => {
    form["d"].value = ((form["height"].value - 2 * form["margin"].value) / form["num_y"].value).toFixed(5);
    updateNumXY();
});


form.addEventListener("submit", (e) => {
    e.preventDefault();

    const fn = form["type"].value == "Grid" ? grid : dot;
    const [r, g, b] = hexToRgb(form["color"].value);
    const width = form["width"].value;
    const height = form["height"].value;
    download("grid.pdf", fn(width, height, form["margin"].value, r, g, b, form["num_x"].value, form["num_y"].value, form["d"].value, form["num_pages"].value));
});
