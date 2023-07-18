import Layout from "../components/layout";
import Reset from "../components/reset";
import Head from 'next/head';
import { useRouter } from 'next/router'
import {postJsonForm, API} from '../utils';
import React, { useState } from "react";
import { useParams } from 'next/navigation'

export const runtime = 'edge';

export default function ResetPage() {
    const router = useRouter()
    const { t } = router.query;

    const [processing, setProcessing] = useState(false);
    const [errorMessage, setErrorMessage] = useState("");
    const [token, setToken] = useState(t);

    // if (!t){
    //     return <>No token found</>
    // }

    let onSubmit = async function (event) {
        event.preventDefault();
        if (processing) {
            setErrorMessage("Processing, please wait before trying again.")
            return
        }

        setProcessing(true);
        const response = await postJsonForm(`${API}/reset`, event, []);
        //console.log(response);
        if (response) {
            // localStorage.setItem('jwt', response);
            window.location.replace('/login');
        } else {
            setErrorMessage("Invalid Token or Token has expired!")
            // window.location.replace('/newreset');
        }
        setProcessing(false);
    }

    return (
        <Layout>
            <Head>
                <title>SynoCommunity - Reset</title>
            </Head>
            <main className="text-gray-700 dark:text-gray-300">
                <Reset errorMessage={errorMessage} setErrorMessage={setErrorMessage} onSubmit={onSubmit} processing={processing} token={token}></Reset>
            </main>
        </Layout>
    )
}
