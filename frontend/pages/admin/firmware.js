import Layout from "../../components/layout";
import Button from "../../components/button";
import TablePaginate from "../../components/table-paginate";
import { useState } from "react";

export default function FirmwarePage({data}) {
    const url = `http://127.0.0.1:8080/api/firmware`

    let [isOpen, setIsOpen] = useState(true);

    function closeModal() {
        setIsOpen(false);
    }

    function openModal() {
        setIsOpen(true);
    }

    const columns = [
        { Header: 'ID', accessor: 'id',},
        { Header: 'version', accessor: 'version',},
        { Header: 'Build', accessor: 'build',},
    ];

    return (
        <Layout>
            <h1>Firmware</h1>
            <TablePaginate columns={columns} url={url}></TablePaginate>
            <Button>Add Firmware</Button>
        </Layout>
    );
}
