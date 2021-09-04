import Layout from "../../components/layout";
import Button from "../../components/button";
import Table from "../../components/table";
import Model from "../../components/model";
import { useState, useRef } from "react";
import { Dialog } from "@headlessui/react";
import { formatImage } from '../../utils';

export async function getServerSideProps({ query }) {
    const url = `http://127.0.0.1:8080/api/screenshot`
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
            pageCount: page+1
        }
    }
}

export default function ScreenshotPage({data, currentPage, pageCount}) {
    let [isOpen, setIsOpen] = useState(true);

    function closeModal() {
        setIsOpen(false);
    }

    function openModal() {
        setIsOpen(true);
    }

    // const data = [
    //     { id: 1, package: "Syncthing", image: "https://packages.synocommunity.com/syncthing/screenshot_1.jpg" },
    // ];

    const columns = [
        { Header: 'ID', accessor: 'id' },
        { Header: 'Package', accessor: 'package' },
        { Header: 'Image', accessor: 'path', Cell: ({ value }) => formatImage(value)},
    ];

    return (
        <Layout>
            <h1>Screenshot</h1>
            <Table columns={columns} data={data} currentPage={currentPage} pageCount={pageCount}></Table>
            <Button>Add Firmware</Button>
        </Layout>
    );
}
