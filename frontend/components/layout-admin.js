import Head from 'next/head'
import Nav from "./nav-admin";
import Link from "next/link";

export default function Layout({ children }) {
    return (
        <>
            <Head>
                <title>SynoCommunity</title>
                <link rel="icon" href="/favicon.ico" />
            </Head>

            <div className="flex min-h-screen flex-col w-full flex-1 md:px-20 dark:bg-gray-800 dark:text-white">
                <nav className="flex flex-wrap flex-row justify-between md:items-center md:space-x-4 py-6 px-6 relative">
                    <Link href="/admin">
                        <a className="block text-xl lg:text-2xl font-bold cursor-pointer">Admin</a>
                    </Link>
                    <Nav className="absolute md:relative top-16 left-0 md:top-0 z-20 md:flex flex-col md:flex-row md:space-x-6
                    font-semibold w-full md:w-auto shadow-md rounded-lg md:rounded-none md:shadow-none md:bg-transparent p-6 pt-0 md:p-0 hidden"></Nav>
                </nav>

                <div className="container mx-auto">
                    { children }
                </div>
            </div>
        </>
      )
}
