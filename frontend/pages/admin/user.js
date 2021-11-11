import Layout from "../../components/layout-admin";
import Model from "../../components/model";
import Button from "../../components/button";
import TablePaginate from "../../components/table-paginate";
import { postJsonForm, API, API_VER } from "../../utils";
import { formatBoolean } from '../../utils';
import { useState } from "react";

export default function UserPage() {
    const url = `${API}/${API_VER}/user`;
    const columns = [
        { Header: "ID", accessor: "id" },
        { Header: "Username", accessor: "username" },
        { Header: "Email", accessor: "email" },
        { Header: "Active", accessor: row => formatBoolean(row.active) },
        { Header: "Confirmed", accessor: "confirmed_at" },
    ]

    const [data, setData] = useState([]);

    return (
        <Layout>
            <h1>Users</h1>
            <TablePaginate columns={columns} url={url} data={data} setData={setData}></TablePaginate>
        </Layout>
    );
}
