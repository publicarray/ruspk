import Head from 'next/head'
import Nav from "./nav";
import Link from "next/link";

export default function Layout({ children }) {
    return (
        <div className="">
            <Head>
                <title>Synocommunity</title>
                {/* <link rel="icon" href="/favicon.ico" /> */}
            </Head>

            <main className="flex flex-col w-full flex-1 md:px-20">
                <nav className="flex flex-wrap flex-row justify-between md:items-center md:space-x-4 bg-white py-6 px-6 relative">
                    <Link href="/admin">
                        <a className="block text-xl lg:text-2xl font-bold cursor-pointer">Admin</a>
                    </Link>
                    <Nav className="absolute md:relative top-16 left-0 md:top-0 z-20 md:flex flex-col md:flex-row md:space-x-6 font-semibold w-full md:w-auto bg-white shadow-md rounded-lg md:rounded-none md:shadow-none md:bg-transparent p-6 pt-0 md:p-0 hidden"></Nav>
                </nav>

                <div className="container mx-auto">
                    { children }
                </div>
            </main>
        </div>
      )
}
