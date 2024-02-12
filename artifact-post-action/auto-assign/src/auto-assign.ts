/**
 * MIT License
 *
 * Copyright (c) 2024 Enalean
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

const NONE_USER_ID = 100;
const ARTIFACT_NOTHING_TO_UPDATE: ArtifactUpdate = {values: []};


export function autoAssign(input: ArtifactChange): ArtifactUpdate {
    if (input.action === "update") {
        return ARTIFACT_NOTHING_TO_UPDATE;
    }

    const contributor_field_content = extractContributorFieldContent(input.current, input.tracker);

    if (contributor_field_content.bind_value_ids.length > 0 && contributor_field_content.bind_value_ids[0] !== NONE_USER_ID) {
        return ARTIFACT_NOTHING_TO_UPDATE;
    }

    return {
        values: [
            {
                field_id: contributor_field_content.field_id,
                bind_value_ids: [input.current.submitted_by],
            }
        ]
    }
}

function extractContributorFieldContent(artifact_state: ArtifactState, tracker: Tracker): SelectBoxField {
    const contributor_field_id = tracker.semantics.contributor?.field_id;

    for (const value of artifact_state.values) {
        if (value.field_id !== contributor_field_id) {
            continue;
        }
        const bind_value_ids = value.bind_value_ids ?? null;
        if (bind_value_ids === null) {
            throw new Error(`The contributor/assignee field does not seem to be a selectbox`);
        }
        return {
            field_id: contributor_field_id,
            bind_value_ids,
        };
    }
    throw new Error(`The tracker does not have the contributor/assignee semantic set`);
}

export interface ArtifactChange {
    action: "create"|"update";
    current: ArtifactState;
    tracker: Tracker;
}

interface ArtifactState {
    submitted_by: number;
    values: Array<{
        field_id: number;
        bind_value_ids?: Array<number>;
    }>
}

interface Tracker {
    semantics: {
        contributor?: {
            field_id: number
        };
    }
}

interface ArtifactUpdate {
    values: Array<SelectBoxField>;
}

interface SelectBoxField {
    field_id: number;
    bind_value_ids: Array<number>;
}
