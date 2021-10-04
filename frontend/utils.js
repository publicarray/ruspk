// retrieve data
export const fetchJson = (url) => fetch(url).then((res) => res.json())

// Endpoint
export const API = "http://127.0.0.1:8080"
export const API_VER = "api"
export const CDN = "http://127.0.0.1:8080"

// table formats
export function formatBoolean(data) {
    switch (data) {
        case true:
            return "Yes"
        case false:
            return "No"
        default:
            return "-"
    }
}

export function formatArray(array) {
    let str = ""
    for (let i = 0; i < array.length -1; i++) {
        str += array[i] + ", ";
    }
    return str += array[array.length -1]
}

import Image from "./components/image"
export function formatImage(url, alt, title) {
    return <Image src={`${CDN}/${url}`} alt={alt} title={title} width="100" height="100"/>
}

// send data
export async function postJson(url, data) {
    const requestOptions = {
        method: "POST",
        // mode: "no-cors", // removes content-type header
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(data),
    };

    let response = await fetch(url, requestOptions)
    if (response === undefined) return;
    if (response.ok === false) {
        console.info("Did not receive a 200 OK response", response)
        return false
    }
    return response.json();
}

export async function postJsonForm(url, event, multipleInputs = []) {
    event.preventDefault();
    // https://www.learnwithjason.dev/blog/get-form-values-as-json/
    // https://developer.mozilla.org/en-US/docs/Web/API/FormData
    // https://caniuse.com/mdn-api_formdata
    const formData = new FormData(event.target);
    // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/fromEntries
    // https://caniuse.com/mdn-javascript_builtins_object_fromentries
    let data = Object.fromEntries(formData.entries());
    for (const field of multipleInputs) {
        data[field] = formData.getAll(field);
    }

    return await postJson(url, data);
}
