import Layout from "../../components/layout";
import Button from "../../components/button";
import TablePaginate from "../../components/table-paginate";
import Model from "../../components/model";
import { useState, useRef } from "react";
import { Dialog } from "@headlessui/react";
import { formatBoolean } from '../../utils';
import { postJsonForm } from "../../utils";

export default function VersionPage({data}) {
    const url = `http://127.0.0.1:8080/api/version`

    // const data = [
    //     { id: 1, package: "python38", version: "3.8.11", revision: "4", beta: "false", services: "", insert_date: "2021-08-31 22:42:37.109035", all_builds_active: "true", install_wizard: "false", upgrade_wizard: "false", startable: "false"  },
    // ];

    const columns = [
        { Header: 'ID', accessor: 'id' },
        { Header: 'Package', accessor: 'package' },
        { Header: 'Upstream Version', accessor: 'upstream_version' },
        { Header: 'Revision', accessor: 'revision' },
        { Header: 'Beta', accessor: 'beta' },
        // { Header: 'Services', accessor: 'services' },
        { Header: 'Insert Date', accessor: 'insert_date' },
        { Header: 'All Builds Active', accessor: row => formatBoolean(row.all_builds_active) },
        { Header: 'Install Wizard', accessor: row => formatBoolean(row.install_wizard) },
        { Header: 'Upgrade Wizard', accessor: row => formatBoolean(row.upgrade_wizard) },
        { Header: 'Startable', accessor: row => formatBoolean(row.startable) },
    ];

    return (
        <Layout>
            <h1>Version</h1>
            <TablePaginate columns={columns} url={url}></TablePaginate>
        </Layout>
    );
}
