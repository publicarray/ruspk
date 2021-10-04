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
                <nav className="flex flex-wrap flex-row justify-between md:items-center md:space-x-4 py-6 px-6 relative bg-gray-100 dark:bg-gray-900 w-full md:px-20">
                    <Link href="/">
                        <a className="block text-gray-400 dark:text-gray-200 text-xl lg:text-2xl font-bold cursor-pointer"><span className="text-blue-600 dark:text-blue-400">Syno</span>Community</a>
                    </Link>
                    <Nav className="absolute md:relative top-16 left-0 md:top-0 z-20 md:flex flex-col md:flex-row md:space-x-6
                    font-semibold w-full md:w-auto shadow-md rounded-lg md:rounded-none md:shadow-none md:bg-transparent p-6 pt-0 md:p-0 hidden"></Nav>
                </nav>

                <div className="container mx-auto">
                    { children }
                </div>
            </div>
            <footer role="contentinfo" className="border-t dark:bg-gray-800 dark:border-gray-600">
                <div className="text-center text-gray-700 dark:text-gray-300 text-sm p-10">
                    <p>Re-Implementation by Sebastian Schmidt.</p>
                    <p>Designed by Antoine Bertin.</p>
                    <p>Maintained by <a className="text-blue-500 dark:text-blue-400 hover:text-blue-700 dark:hover:dark:text-blue-500  hover:underline" href="https://github.com/SynoCommunity?tab=members">SynoCommunity</a> with the help of <a className="text-blue-500 dark:text-blue-400 hover:text-blue-700 dark:hover:dark:text-blue-500  hover:underline" href="https://github.com/SynoCommunity/spksrc/graphs/contributors">contributors</a>.</p>
                    <p>Code licensed under <a className="text-blue-500 dark:text-blue-400 hover:text-blue-700 dark:hover:dark:text-blue-500  hover:underline" href="https://github.com/SynoCommunity/spkrepo/blob/master/LICENSE" target="_blank">MIT</a>.</p>
                </div>
            </footer>
        </>
      )
}
