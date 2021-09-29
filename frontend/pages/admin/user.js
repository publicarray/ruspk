import Layout from "../../components/layout";
import Model from "../../components/model";
import Button from "../../components/button";
import TablePaginate from "../../components/table-paginate";
import { useState } from "react";
import { postJsonForm } from "../../utils";
import { formatBoolean } from '../../utils';

export default function ArchitecturePage() {
    const url = `http://127.0.0.1:8080/api/user`;
    const columns = [
        { Header: "ID", accessor: "id" },
        { Header: "Username", accessor: "username" },
        { Header: "Email", accessor: "email" },
        { Header: "Active", accessor: row => formatBoolean(row.active) },
        { Header: "Confirmed", accessor: "confirmed_at" },
    ];

    return (
        <Layout>
            <h1>Users</h1>
            <TablePaginate columns={columns} url={url}></TablePaginate>
        </Layout>
    );
}
