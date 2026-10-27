    # SigmaOS-mirror

    Mirror speed tester & automatic selector

    Part of the **SigmaOS** sovereign operating-system ecosystem — a multi-repository
    distribution modelled on Linux Mint and Omarchy Linux. See the parent
    [`SigmaOS`](https://github.com/AaryanSinghChauhan09/SigmaOS) repository for the
    full component index.

    ## Upstream inspirations
    - https://github.com/omacom/omarchy-mirror

    ## Design principles applied
    - **Plan Before Apply**
- **Deterministic Testing**
- **KISS**
- **Least Astonishment**
- **Fail-Fast**

    ## Usage
    Verbs: `list, score-plan, select, write-plan`

    ```
    $ sigma-mirror list --file mirrors.txt
$ sigma-mirror score-plan --file mirrors.txt --output scores.json
$ sigma-mirror select --file mirrors.txt --scores scores.json --country DE --count 3
$ sigma-mirror write-plan --selected selected.txt --output /etc/apt/mirrorlist

    ```

    ## Build & test
    ```
    make build
    make test
    ```

    ## Licence
    GNU General Public License v3.0 (FOSS).
