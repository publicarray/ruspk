import Link from "next/link";
import ThemeChanger from "./theme-changer"

export default function Nav() {
    return (
        <nav className="bg-gray-100 dark:bg-gray-900 flex flex-wrap md:flex-row justify-between md:items-center md:space-x-4 py-6 px-6 relative">
            <Link href="/admin">
                <a className="block text-xl lg:text-2xl font-bold cursor-pointer">
                    <span className="text-blue-600 dark:text-blue-400">Syno</span><span className="text-gray-500 dark:text-gray-200">Community</span> - Admin
                </a>
            </Link>

            <div className="flex md:flex-row flex-wrap text-gray-700 dark:text-gray-300">
                <Link href="/">
                    <a className="leading-loose md:h-12 px-4 py-2 rounded-lg transition-all hover:bg-gray-200 dark:hover:bg-gray-700">Home</a>
                </Link>

                <Link href="/admin/user/">
                    <a className="leading-loose md:h-12 px-4 py-2 rounded-lg transition-all hover:bg-gray-200 dark:hover:bg-gray-700">User</a>
                </Link>

                <Link href="/admin/architecture/">
                    <a className="leading-loose md:h-12 px-4 py-2 rounded-lg transition-all hover:bg-gray-200 dark:hover:bg-gray-700">Architecture</a>
                </Link>

                <Link href="/admin/firmware/">
                    <a className="leading-loose md:h-12 px-4 py-2 rounded-lg transition-all hover:bg-gray-200 dark:hover:bg-gray-700">Firmware</a>
                </Link>

                <Link href="/admin/screenshot/">
                    <a className="leading-loose md:h-12 px-4 py-2 rounded-lg transition-all hover:bg-gray-200 dark:hover:bg-gray-700">Screenshot</a>
                </Link>

                <Link href="/admin/package/">
                    <a className="leading-loose md:h-12 px-4 py-2 rounded-lg transition-all hover:bg-gray-200 dark:hover:bg-gray-700">Package</a>
                </Link>

                <Link href="/admin/version/">
                    <a className="leading-loose md:h-12 px-4 py-2 rounded-lg transition-all hover:bg-gray-200 dark:hover:bg-gray-700">Version</a>
                </Link>

                <Link href="/admin/build/">
                    <a className="leading-loose md:h-12 px-4 py-2 rounded-lg transition-all hover:bg-gray-200 dark:hover:bg-gray-700">Build</a>
                </Link>
                <ThemeChanger></ThemeChanger>
            </div>
        </nav>
    );
}
