import Link from "next/link";

export default function Nav() {
    return (
        <div className="md:flex justify-around space-x-4 text-gray-700 dark:text-gray-300">
            {/* <Link href="/admin">
                <a className="block py-1  hover:underline">Admin</a>
            </Link> */}

            <Link href="/admin/user/">
                <a className="block py-1  hover:underline">User</a>
            </Link>

            <Link href="/admin/architecture/">
                <a className="block py-1  hover:underline">Architecture</a>
            </Link>

            <Link href="/admin/firmware/">
                <a className="block py-1  hover:underline">Firmware</a>
            </Link>

            <Link href="/admin/screenshot/">
                <a className="block py-1  hover:underline">Screenshot</a>
            </Link>

            <Link href="/admin/package/">
                <a className="block py-1  hover:underline">Package</a>
            </Link>

            <Link href="/admin/version/">
                <a className="block py-1  hover:underline">Version</a>
            </Link>

            <Link href="/admin/build/">
                <a className="block py-1  hover:underline">Build</a>
            </Link>
        </div>
    );
}
