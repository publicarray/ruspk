import Layout from "../components/layout";
import Image from "../components/image";
import Head from 'next/head'

function Error({ statusCode }) {
    return (
        <Layout>
            <Head>
                <title>SynoCommunity - Error</title>
            </Head>
            <main className="text-gray-700 dark:text-gray-300 max-w-6xl mx-auto mt-4">
                <h1 className="text-5xl text-center">
                    {statusCode
                    ? `An error ${statusCode} occurred on server`
                    : 'An error occurred on client'}
                </h1>
            </main>
        </Layout>
  )
}

Error.getInitialProps = ({ res, err }) => {
    const statusCode = res ? res.statusCode : err ? err.statusCode : 404
    return { statusCode }
}

  export default Error
