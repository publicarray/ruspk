import Layout from "../components/layout";
import Head from 'next/head';

export const config = {
    runtime: 'experimental-edge',
}

export default function LogoutPage() {
    if (typeof window !== "undefined") {
        localStorage.removeItem("jwt");
    }
    return (
        <Layout>
            <Head>
                <title>SynoCommunity - Logout</title>
            </Head>
            <main className="text-gray-700 dark:text-gray-300">
                <p>You are now logged out!</p>
            </main>
        </Layout>
    )
}
