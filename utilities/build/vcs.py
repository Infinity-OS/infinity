# Obtain the revision number from the Git repository.
def revision_id():
	from subprocess import Popen, PIPE
	git = Popen(['git', 'rev-parse', '--short', 'HEAD'], stdout = PIPE, stderr = PIPE)
	revision = git.communicate()[0].strip()

	if git.returncode != 0:
		return None

	return revision
