station test_simple_sarif[train]

    # 1. Arrange: Set train settings
    train[flaws]        = []
    train[file_path]    = ../run_tests.rs
    train[file_manage]  = yes
    train[app_logs_dir] = /data/logs

    # 2. Create fully valid flaw
    flaw = [
        tool_name        = CLI
        tool_version     = 0.1.0
        tool_url         = https://bitbucket.org/AliasAPI/micromanager
        rule_id          = TT001
        rule_name        = ONE_FUNC_PER_FILE 
        short_description= Only allow one function per file
        full_description = There are too many functions in the file
        severity_level   = critical
        start_line       = 0
        end_line         = 0
        start_column     = 0
        end_column       = 0
        artifact_url     = file:// + train[file_path]
        help_url         = https://bitbucket.org/AliasAPI/microscanner/rules/TT001.md
        message          = There should only be one function per file 
        prompt           = Chunk the code so that there is only one top-level function in the file
    ]

    # 3. Set up expected SARIF result array
    sarif = [
        flaw + [
            locations = [
                physicalLocation = [
                    artifactLocation = [ uri = flaw.artifact_url ]
                    region = [
                        startLine   = flaw.start_line
                        endLine     = flaw.end_line
                        startColumn = flaw.start_column
                        endColumn   = flaw.end_column
                    ]
                ]
            ]
        ]
    ]

    # 4. Add the flaw to train
    train[flaws] = append flaw 

    # 5. Run pipeline
    train = track_sarif[train]

    # 6. Check the results equal 
    assert train[sarif] == sarif

    # 7. Return train
    train
