import Layout from "../components/layout";
import Image from "../components/image";
import Head from 'next/head'

// export const config = {
//     runtime: 'experimental-edge',
// }

export default function NotFound() {
    return (
        <Layout>
            <Head>
                <title>SynoCommunity - NotFound</title>
            </Head>
            <main className="text-gray-700 dark:text-gray-300 max-w-6xl mx-auto mt-4">
                <h1 className="text-5xl text-center">404 - Page Not Found</h1>
            </main>
        </Layout>
  )
}
