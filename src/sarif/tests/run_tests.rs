
station run_tests[train]
    if train[wreck] is set
        return train

    train[log] append [3, run_$tool:]

    if train[file_manage] != yes OR
       train[contained] != yes
        return train

    if train[task] != test
        return train

    Add all of the test_ files in the current directory to an array

    Randomize the array of tests

    Run each test which will return SARIF

    return train