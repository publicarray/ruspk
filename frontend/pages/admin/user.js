import Layout from "../../components/layout-admin";
import Model from "../../components/model";
import Button from "../../components/button";
import TablePaginate from "../../components/table-paginate";
import { postJsonForm, API, API_VER } from "../../utils";
import { formatBoolean } from '../../utils';
import { useState } from "react";
import { createColumnHelper } from "@tanstack/react-table";

export default function UserPage() {
    const url = `${API}/${API_VER}/user`;
    const columnHelper = createColumnHelper();
    const columns = [
        columnHelper.accessor("id"),
        columnHelper.accessor("username"),
        columnHelper.accessor("email"),
        columnHelper.accessor("active", {
            cell: (info) => formatBoolean(info.getValue())
        }),
        columnHelper.accessor("confirmed_at", {
            header: "Confirmed",
        }),
    ];

    const [data, setData] = useState([]);

    return (
        <Layout>
            <h1>Users</h1>
            <TablePaginate columns={columns} url={url} data={data} setData={setData}></TablePaginate>
        </Layout>
    );
}
