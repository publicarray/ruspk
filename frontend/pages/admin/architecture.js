import Layout from "../../components/layout";
import Button from "../../components/button";
import Table from "../../components/table";
import Model from "../../components/model";
import { useState, useRef } from "react";
import { Dialog } from "@headlessui/react";

export async function getServerSideProps({ query }) {
    const url = `http://127.0.0.1:8080/api/architecture`
    const page = query.page || 1; //if page empty we request the first page
    const res = await fetch(`${url}?page=${page}&size=15`)
    const data = await res.json()

    if (!data) {
        return {
            notFound: true,
        }
    }

    return {
        props: {
            data,
            currentPage: page,
            pageCount: 100
        }
    }
}

export default function ArchitecturePage({data, currentPage, pageCount}) {
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
            <Table columns={columns} data={data} currentPage={currentPage} pageCount={pageCount}></Table>
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
