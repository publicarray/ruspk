// import useSWR from 'swr'

export const fetch_json = (url) => fetch(url).then((res) => res.json())

export const client_get_json = (url) => useSWR(url, fetch_json)

export function formatBoolean(data) {
    if ( data === true ) {
        return "Yes"
    } else if (data === false) {
        return "No"
    }
    return "-"
}

export function formatArray(array) {
    let str = ""
    for (let i = 0; i < array.length -1; i++) {
        str += array[i] + ", ";
    }
    return str += array[array.length -1]
}

export function formatImage(url, alt, title) {
    return <img src={url} alt={alt} title={title}></img>
}

export async function postJson(url, data) {
    const requestOptions = {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(data),
    };

    let response = await fetch(url, requestOptions)
        .catch((err) => console.error(err));
    return response.json().catch((err) => console.error(err));
}
