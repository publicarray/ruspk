import Layout from "../components/layout";
import Head from 'next/head'
import useSWR from 'swr'
import { fetchJson, API, API_VER, CDN } from "../utils";
import Link from 'next/link'

export default function Packages(props) {

    const url = `${API}/${API_VER}/package?size=60`

    let { data, error } = useSWR(`${url}`, fetchJson);
    let isLoading = !error && !data;
    let isError = !error;
    if (isError) {
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

            <main className="flex flex-wrap">
                {data.map(row => {
                    return (
                        <div key={row.name} className="text-black bg-gray-100 hover:bg-gray-200 dark:text-gray-300 dark:bg-gray-900 dark:hover:bg-gray-700 p-4 m-2 flex-auto rounded-md w-64">
                            <Link href={{
                                    pathname: '/package/[name]',
                                    query: { name: row.name },
                                }}>
                                <a>
                                    <h2 className="text-2xl whitespace-nowrap text-center mb-2">{row.displayname}</h2>
                                    <img className="rounded-xl mx-auto mb-2" src={`${CDN}/${row.name}/${row.revision}/icon256.png`} alt={row.name} />
                                    {/* <img className="rounded-xl mx-auto mb-2" src="https://images.placeholders.dev/?width=100&height=100&bgColor=%23313131" alt={row.name} /> */}
                                    <p className="text-center text-gray-500 dark:text-gray-400 text-sm">v{row.version}-{row.revision}</p>
                                    {/* <p className="mb-2">{row.author}</p> */}
                                    <p className="mb-2">{row.description}</p>
                                </a>
                            </Link>
                        </div>
                    )
                })}
            </main>
        </Layout>
  )
}
