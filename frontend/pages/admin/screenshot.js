import Layout from "../../components/layout";
import Button from "../../components/button";
import TablePaginate from "../../components/table-paginate";
import Model from "../../components/model";
import { useState, useRef } from "react";
import { Dialog } from "@headlessui/react";
import { formatImage } from '../../utils';

export default function ScreenshotPage({data}) {
    const url = `http://127.0.0.1:8080/api/screenshot`
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
            <TablePaginate columns={columns} url={url}></TablePaginate>
            <Button>Add Firmware</Button>
        </Layout>
    );
}
