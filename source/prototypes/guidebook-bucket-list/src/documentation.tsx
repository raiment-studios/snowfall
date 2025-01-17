import React, { JSX } from 'react';

export function Documentation(): JSX.Element {
    return (
        <>
            <style>{`
        .documentation {
            max-width: 86ch;
            padding: 3ch 0 16ch;
            color: #555;

            h2 {
                margin-top: 32px;
                color: #227587;
            }
        }
        `}</style>
            <div className="documentation" style={{}}>
                <h1 style={{ marginBottom: 48 }}>Guidebook: Bucket List</h1>
                <p>
                    <strong>Guidebook: Bucket List</strong>
                    is a simple application for tracking a life bucket list in a YAML file hosted on
                    GitHub.
                </p>
                <p>
                    It allows to you to track your goals, their priority, their status, as well as
                    notes about them. As the data is stored in GitHub and in a standard YAML format,
                    you have complete control over your data as well as a complete history of
                    changes to it.
                </p>
                <h2>Usage</h2>
                <p>
                    <strong>Status</strong> -- pretty simple: it's "todo", "wip" (work in progress),
                    or "done".
                </p>
                <p>
                    <strong>Category</strong> -- an arbitrary label to group different items
                    together.
                </p>
                <p>
                    <strong>Value</strong> -- a 1 to 10 rating of how important completing this item
                    is. A value of 10 means you really do not want to "kick the bucket" without
                    completing this item.
                </p>
                <p>
                    <strong>Year/Month</strong> --
                </p>
                <p>
                    <strong>Rating</strong> --
                </p>
                <p>
                    <strong>Review</strong> --
                </p>
            </div>
        </>
    );
}
