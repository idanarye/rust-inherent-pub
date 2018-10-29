import vim
from omnipytent import *
from omnipytent.ext.idan import *


@task
def compile(ctx):
    cargo['build', '-q', '--examples'] & ERUN.bang


@task
def run(ctx):
    cargo['run', '-q', '--example', 'example'] & ERUN.bang
