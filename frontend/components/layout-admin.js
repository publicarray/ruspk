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

            <div className="flex min-h-screen flex-col w-full dark:bg-gray-800 dark:text-white">
                <Nav></Nav>
                <div className="container mx-auto">
                    { children }
                </div>
            </div>
        </>
      )
}
