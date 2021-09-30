import Layout from "../components/layout";
import Head from 'next/head'
import useSWR from 'swr'
import { fetchJson } from "../utils";
import Link from 'next/link'

export default function Packages(props) {

    const url = `http://127.0.0.1:8080/api/package`

    let { data, error } = useSWR(`${url}`, fetchJson);
    let isLoading = !error && !data;
    let isError = !error;

    if (error) {
        console.error(error)
    }
    if (isLoading) {
        return <p>Loading...</p>
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
                                    <h2 className="text-2xl whitespace-nowrap text-center mb-2">{row.name}</h2>
                                    {/* <h2 className="text-2xl whitespace-nowrap text-center">{row.displayname}</h2> */}
                                    <img className="rounded-xl mx-auto mb-2" src="https://images.placeholders.dev/?width=100&height=100&bgColor=%23313131" alt={row.name} />
                                    {/* <p className="text-center text-gray-500 text-sm">{row.version}</p> */}
                                    <p className="text-center text-gray-500 dark:text-gray-400 text-sm">v1.0.0-3</p>
                                    {/* <p className="mb-2">{row.author}</p> */}
                                    <p className="mb-2">Ad nisi nulla nisi enim qui sint ullamco officia deserunt. Pariatur elit Lorem dolore ipsum est id non. Aliquip adipisicing sit ut ullamco in ea ut. Cillum labore qui cupidatat dolor est amet incididunt ea nostrud.</p>
                                </a>
                            </Link>
                        </div>
                    )
                })}
            </main>
        </Layout>
  )
}
