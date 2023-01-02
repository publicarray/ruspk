import Layout from "../components/layout";
import Login from "../components/login";
import Head from 'next/head';
import {postJsonForm, API} from '../utils';
import React, { useState } from "react";

export const config = {
    runtime: 'experimental-edge',
}

export default function LoginPage() {
    const [processing, setProcessing] = useState(false);
    const [errorMessage, setErrorMessage] = useState("");
    let onSubmit = async function (event) {
        event.preventDefault();
        if (processing) {
            setErrorMessage("Processing, please wait before trying again.")
            return
        }

        setProcessing(true);
        const response = await postJsonForm(`${API}/login`, event, []);
        //console.log(response);
        if (response) {
            localStorage.setItem('jwt', response);
            window.location.replace('/admin');
        } else {
            setErrorMessage("Invalid Credentials")
        }
        setProcessing(false);
    }

    return (
        <Layout>
            <Head>
                <title>SynoCommunity - Login</title>
            </Head>
            <main className="text-gray-700 dark:text-gray-300">
                <Login errorMessage={errorMessage} setErrorMessage={setErrorMessage} onSubmit={onSubmit} processing={processing}></Login>
            </main>
        </Layout>
    )
}
