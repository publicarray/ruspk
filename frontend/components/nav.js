import Link from "next/link";
import ThemeChanger from "./theme-changer"

export default function Nav() {
    return (
        <nav className="bg-gray-100 dark:bg-gray-900 flex flex-wrap md:flex-row justify-between md:items-center md:space-x-4 py-6 px-6 relative">
            <Link href="/">
                <a className="block text-gray-500 dark:text-gray-200 text-xl lg:text-2xl font-bold cursor-pointer">
                    <span className="text-blue-600 dark:text-blue-400">Syno</span>Community
                </a>
            </Link>

            <div className="flex md:flex-row flex-wrap text-gray-700 dark:text-gray-300">
                <Link href="/">
                    <a className="leading-loose md:h-12 px-4 py-2 rounded-lg transition-all hover:bg-gray-200 dark:hover:bg-gray-700">Home</a>
                </Link>
                <Link href="/packages">
                    <a className="leading-loose md:h-12 px-4 py-2 rounded-lg transition-all hover:bg-gray-200 dark:hover:bg-gray-700">Packages</a>
                </Link>
                <ThemeChanger></ThemeChanger>
            </div>
        </nav>
    );
}
