import React, { useState } from 'react';
import Table from './table';
// import { useRouter } from "next/router"

export default function TablePaginate({columns, url}) {
    // const router = useRouter()
    // const page = router.query.page || 1
    // const [pageIndex, setPageIndex] = useState(page);
    const [pageIndex, setPageIndex] = useState(1);

    return (
        <div>
            <Table pageIndex={pageIndex} url={url} columns={columns}/>
            <div style={{ display: 'none' }}><Table pageIndex={pageIndex + 1} url={url} columns={columns}/></div>
            <button className="py-1 px-3 text-white rounded-lg bg-blue-500 hover:bg-blue-700" onClick={() => setPageIndex(pageIndex - 1)}>Previous</button>
            <button className="py-1 px-3 text-white rounded-lg bg-blue-500 hover:bg-blue-700" onClick={() => setPageIndex(pageIndex + 1)}>Next</button>
        </div>
    )
}
