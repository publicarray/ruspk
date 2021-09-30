import Link from "next/link";

export default function Nav() {
    return (
        <div className="md:flex justify-around space-x-4 text-gray-700 dark:text-gray-300">
            <Link href="/">
                <a className="block py-1  hover:underline">Home</a>
            </Link>
            <Link href="/packages">
                <a className="block py-1  hover:underline">Packages</a>
            </Link>
        </div>
    );
}
