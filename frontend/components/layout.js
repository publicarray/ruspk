import Head from 'next/head'
import Nav from "./nav";
import Link from "next/link";

export default function Layout({ children }) {
    return (
        <>
            <Head>
                <title>SynoCommunity</title>
                <link rel="icon" href="/favicon.ico" />
            </Head>

            <div className="flex min-h-screen flex-col w-full dark:bg-gray-800 dark:text-white">
                <Nav></Nav>
                <div>
                    { children }
                </div>
            </div>
            <footer role="contentinfo" className="border-t dark:bg-gray-800 dark:border-gray-600">
                <div className="text-center text-gray-700 dark:text-gray-300 text-sm p-10">
                    <p>Re-Implementation by Sebastian Schmidt.</p>
                    <p>Designed by Antoine Bertin.</p>
                    <p>Maintained by <a className="text-blue-500 dark:text-blue-400 hover:text-blue-700 dark:hover:dark:text-blue-500  hover:underline" href="https://github.com/SynoCommunity?tab=members">SynoCommunity</a> with the help of <a className="text-blue-500 dark:text-blue-400 hover:text-blue-700 dark:hover:dark:text-blue-500  hover:underline" href="https://github.com/SynoCommunity/spksrc/graphs/contributors">contributors</a>.</p>
                    <p>Code licensed under <a className="text-blue-500 dark:text-blue-400 hover:text-blue-700 dark:hover:dark:text-blue-500  hover:underline" href="https://github.com/SynoCommunity/spkrepo/blob/master/LICENSE">MIT</a>.</p>
                </div>
            </footer>
        </>
      )
}
