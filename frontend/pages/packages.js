import Layout from "../components/layout";
import Head from 'next/head'
import useSWR from 'swr'
import { fetchJson, API, API_VER, CDN } from "../utils";
import Link from 'next/link'
import { useState } from "react";
import Button from "../components/button";
import Image from "../components/image";

export default function Packages(props) {

    const [pageIndex, setPageIndex] = useState(1);
    const url = `${API}/${API_VER}/package?page=${pageIndex}&size=60`
    // const url = `${API}/${API_VER}/package?page=${pageIndex}&size=50&q=${query}`

    let { data, error } = useSWR(`${url}`, fetchJson);
    let isLoading = !error && !data;
    let isError = !error;
    if (isError && error != undefined) {
        console.error(error)
    }
    if (isLoading) {
        return <p>Loading...</p>
    }
    if (data == undefined) {
        data = []
    }

    return (
        <Layout>
            <Head>
                <title>SynoCommunity</title>
            </Head>

            <main className="flex flex-wrap overflow-x-hidden">
                {data.map(row => {
                    return (
                        <Link key={row.name}
                                href={{
                                pathname: '/package/[name]',
                                query: { name: row.name },
                            }}>
                            <a className="text-black bg-gray-100 hover:bg-gray-200 dark:text-gray-300 dark:bg-gray-900 dark:hover:bg-gray-700 p-4 m-2 flex-auto rounded-md md:w-72">
                                <div>
                                    <h2 className="text-2xl text-center mb-2">{row.displayname}</h2>
                                    <div className="flex justify-center">
                                        <Image className="rounded-xl mb-2" src={`${CDN}/${row.name}/${row.revision}/icon256.png`} width="256" height="256" title={row.name} alt=""/>
                                    </div>
                                    <p className="text-center text-gray-500 dark:text-gray-400 text-sm">v{row.version}-{row.revision}</p>
                                    {/* <p className="mb-2">{row.author}</p> */}
                                    {/* <p className="mb-2">{row.description}</p> */}
                                </div>
                            </a>
                        </Link>
                    )
                })}
            </main>
            <div className="flex mb-4">
                <Button className="mr-auto" onClick={() => setPageIndex(pageIndex - 1)}>Previous</Button>
                <Button className="ml-auto" onClick={() => setPageIndex(pageIndex + 1)}>Next</Button>
            </div>
        </Layout>
  )
}
