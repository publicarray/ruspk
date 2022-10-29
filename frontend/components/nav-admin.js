import Link from "next/link";
import ThemeChanger from "./theme-changer"

export default function Nav() {
    return (
        <nav className="bg-gray-100 dark:bg-gray-900 flex flex-wrap md:flex-row justify-between md:items-center md:space-x-4 py-6 px-6 relative">
            <Link
                href="/admin"
                className="block text-xl lg:text-2xl font-bold cursor-pointer"
                legacyBehavior>
                <span className="text-blue-600 dark:text-blue-400">Syno</span><span className="text-gray-500 dark:text-gray-200">Community</span>- Admin
            </Link>

            <div className="flex md:flex-row flex-wrap text-gray-700 dark:text-gray-300">
                <Link
                    href="/"
                    className="leading-loose md:h-12 px-4 py-2 rounded-lg transition-all hover:bg-gray-200 dark:hover:bg-gray-700">
                    Home
                </Link>

                <Link
                    href="/admin/user/"
                    className="leading-loose md:h-12 px-4 py-2 rounded-lg transition-all hover:bg-gray-200 dark:hover:bg-gray-700">
                    User
                </Link>

                <Link
                    href="/admin/architecture/"
                    className="leading-loose md:h-12 px-4 py-2 rounded-lg transition-all hover:bg-gray-200 dark:hover:bg-gray-700">
                    Architecture
                </Link>

                <Link
                    href="/admin/firmware/"
                    className="leading-loose md:h-12 px-4 py-2 rounded-lg transition-all hover:bg-gray-200 dark:hover:bg-gray-700">
                    Firmware
                </Link>

                <Link
                    href="/admin/screenshot/"
                    className="leading-loose md:h-12 px-4 py-2 rounded-lg transition-all hover:bg-gray-200 dark:hover:bg-gray-700">
                    Screenshot
                </Link>

                <Link
                    href="/admin/package/"
                    className="leading-loose md:h-12 px-4 py-2 rounded-lg transition-all hover:bg-gray-200 dark:hover:bg-gray-700">
                    Package
                </Link>

                <Link
                    href="/admin/version/"
                    className="leading-loose md:h-12 px-4 py-2 rounded-lg transition-all hover:bg-gray-200 dark:hover:bg-gray-700">
                    Version
                </Link>

                <Link
                    href="/admin/build/"
                    className="leading-loose md:h-12 px-4 py-2 rounded-lg transition-all hover:bg-gray-200 dark:hover:bg-gray-700">
                    Build
                </Link>
                <ThemeChanger></ThemeChanger>
            </div>
        </nav>
    );
}
