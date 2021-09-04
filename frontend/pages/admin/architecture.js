import Layout from "../../components/layout";
import Button from "../../components/button";
import TablePaginate from "../../components/table-paginate";
import { useState } from "react";
import { Dialog } from "@headlessui/react";

export default function ArchitecturePage({data}) {
    const url = `http://127.0.0.1:8080/api/architecture`

    let [isOpen, setIsOpen] = useState(true);

    function closeModal() {
        setIsOpen(false);
    }

    function openModal() {
        setIsOpen(true);
    }

    const columns = [
        { Header: 'ID', accessor: 'id',},
        { Header: 'Firmware', accessor: 'code',},
    ];

    return (
        <Layout>
            <h1>Architecture</h1>
            <TablePaginate columns={columns} url={url}></TablePaginate>
            <Button>Add Architecture</Button>
            {/* <Model title="Hello">
                This will permanently deactivate your account
            </Model> */}
            <button
                type="button"
                onClick={openModal}
                className="px-4 py-2 text-sm font-medium text-white bg-black rounded-md bg-opacity-20 hover:bg-opacity-30 focus:outline-none focus-visible:ring-2 focus-visible:ring-white focus-visible:ring-opacity-75"
            >
                Open dialog
            </button>
        </Layout>
    );
}
